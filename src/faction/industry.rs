use bevy::prelude::*;
use std::collections::HashMap;

use crate::faction::stockpile::TileStockpile;
use crate::faction::upkeep::TileOperatingGrid;
use crate::map::hex::HexCoord;
use crate::map::resource::ResourceAmounts;

/// 二次中間財 (Manufactured Goods)
/// (参照: docs/specs/RESOURCE_ECONOMY_SYSTEM.md)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ManufacturedGood {
    /// 基礎機械部品 (構造鉱物 + エネルギー)
    StandardParts,
    /// 電子・論理コア (機能性鉱物/科学 + エネルギー)
    LogicCores,
    /// 耐熱複合装甲板 (構造鉱物 + 機能性鉱物)
    CompositeArmor,
    /// 規格化レーション (バイオ原質)
    StandardRations,
}

/// 兵站消費財 (Consumables)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MilitaryConsumable {
    /// C規格軍用弾薬
    Munitions,
    /// 推進剤・燃料
    PropellantFuel,
    /// 野戦交換部品
    FieldSpares,
}

/// 工業加工レシピのID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RecipeId {
    /// 構造鉱物とエネルギーから基礎機械部品を製造
    ProduceStandardParts,
    /// 機械部品とエネルギーからC規格軍用弾薬を製造
    ProduceMunitions,
    /// バイオマスとエネルギーから燃料・推進剤を製造
    ProduceFuel,
    /// 機械部品とエネルギーから野戦交換部品を製造
    ProduceFieldSpares,
}

/// 各レシピの入出力定義
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct IndustryRecipe {
    pub id: RecipeId,
    pub input_cost: ResourceAmounts,
    pub input_parts: f32,
    pub output_parts: f32,
    pub output_munitions: f32,
    pub output_fuel: f32,
    pub output_spares: f32,
}

impl IndustryRecipe {
    pub fn get(id: RecipeId) -> Self {
        match id {
            RecipeId::ProduceStandardParts => Self {
                id,
                input_cost: ResourceAmounts::new(0.0, 4.0, 2.0, 0.0),
                input_parts: 0.0,
                output_parts: 2.0,
                output_munitions: 0.0,
                output_fuel: 0.0,
                output_spares: 0.0,
            },
            RecipeId::ProduceMunitions => Self {
                id,
                input_cost: ResourceAmounts::new(0.0, 2.0, 3.0, 0.0),
                input_parts: 1.0,
                output_parts: 0.0,
                output_munitions: 5.0,
                output_fuel: 0.0,
                output_spares: 0.0,
            },
            RecipeId::ProduceFuel => Self {
                id,
                input_cost: ResourceAmounts::new(3.0, 0.0, 4.0, 0.0),
                input_parts: 0.0,
                output_parts: 0.0,
                output_munitions: 0.0,
                output_fuel: 4.0,
                output_spares: 0.0,
            },
            RecipeId::ProduceFieldSpares => Self {
                id,
                input_cost: ResourceAmounts::new(0.0, 1.0, 2.0, 0.0),
                input_parts: 2.0,
                output_parts: 0.0,
                output_munitions: 0.0,
                output_fuel: 0.0,
                output_spares: 2.0,
            },
        }
    }
}

/// タイルごとの二次財・兵站消費財在庫
#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub struct AdvancedStock {
    pub standard_parts: f32,
    pub munitions: f32,
    pub fuel: f32,
    pub field_spares: f32,
}

/// ターン進行時に各工場の加工処理を実行するシステム
pub fn process_turn_production_system(
    mut messages: MessageReader<crate::turn::AdvanceTurnMessage>,
    mut industry_grid: ResMut<IndustryGrid>,
    mut stockpile: ResMut<TileStockpile>,
    operating_grid: Res<TileOperatingGrid>,
) {
    for _ in messages.read() {
        industry_grid.process_turn_production(&mut stockpile, &operating_grid);
    }
}

/// タイル上の工業施設・工場
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
pub struct ManufacturingPlant {
    pub capacity: f32,
    pub active_recipe: Option<RecipeId>,
}

impl Default for ManufacturingPlant {
    fn default() -> Self {
        Self {
            capacity: 1.0,
            active_recipe: Some(RecipeId::ProduceStandardParts),
        }
    }
}

/// タイルごとの工場と二次財在庫を管理するリソース
#[derive(Resource, Default, Debug, Clone)]
pub struct IndustryGrid {
    pub plants: HashMap<HexCoord, ManufacturingPlant>,
    pub advanced_stocks: HashMap<HexCoord, AdvancedStock>,
}

impl IndustryGrid {
    pub fn set_plant(&mut self, coord: HexCoord, plant: ManufacturingPlant) {
        self.plants.insert(coord, plant);
    }

    pub fn get_advanced_stock(&self, coord: &HexCoord) -> AdvancedStock {
        self.advanced_stocks.get(coord).copied().unwrap_or_default()
    }

    /// ターン更新時に各工場の加工処理を実行
    pub fn process_turn_production(
        &mut self,
        stockpile: &mut TileStockpile,
        operating_grid: &TileOperatingGrid,
    ) {
        for (&coord, plant) in &self.plants {
            let Some(recipe_id) = plant.active_recipe else {
                continue;
            };

            let efficiency = operating_grid.get_state(&coord).efficiency;
            if efficiency <= 0.0 {
                continue;
            }

            let recipe = IndustryRecipe::get(recipe_id);
            let effective_capacity = plant.capacity * efficiency;

            let tile_raw = stockpile.stock.entry(coord).or_default();
            let adv_stock = self.advanced_stocks.entry(coord).or_default();

            // 必要原料の計算
            let req_energy = recipe.input_cost.energy * effective_capacity;
            let req_ore = recipe.input_cost.ore * effective_capacity;
            let req_bio = recipe.input_cost.biomass * effective_capacity;
            let req_parts = recipe.input_parts * effective_capacity;

            // 原料が足りている比率を算出（ボトルネックに基づく実行レート）
            let mut rate = 1.0f32;
            if req_energy > 0.0 {
                rate = rate.min(tile_raw.energy / req_energy);
            }
            if req_ore > 0.0 {
                rate = rate.min(tile_raw.ore / req_ore);
            }
            if req_bio > 0.0 {
                rate = rate.min(tile_raw.biomass / req_bio);
            }
            if req_parts > 0.0 {
                rate = rate.min(adv_stock.standard_parts / req_parts);
            }
            let run_rate = rate.clamp(0.0, 1.0);

            if run_rate > 0.0 {
                // 原料の消費
                tile_raw.energy -= req_energy * run_rate;
                tile_raw.ore -= req_ore * run_rate;
                tile_raw.biomass -= req_bio * run_rate;
                adv_stock.standard_parts -= req_parts * run_rate;

                // 生成物の加算
                let out_scale = effective_capacity * run_rate;
                adv_stock.standard_parts += recipe.output_parts * out_scale;
                adv_stock.munitions += recipe.output_munitions * out_scale;
                adv_stock.fuel += recipe.output_fuel * out_scale;
                adv_stock.field_spares += recipe.output_spares * out_scale;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_turn_production_standard_parts() {
        let mut ind = IndustryGrid::default();
        let mut stockpile = TileStockpile::default();
        let op_grid = TileOperatingGrid::default();

        let coord = HexCoord::new(2, 3);
        ind.set_plant(
            coord,
            ManufacturingPlant {
                capacity: 1.0,
                active_recipe: Some(RecipeId::ProduceStandardParts),
            },
        );

        // 原料十分
        stockpile
            .stock
            .insert(coord, ResourceAmounts::new(0.0, 10.0, 10.0, 0.0));

        ind.process_turn_production(&mut stockpile, &op_grid);

        let adv = ind.get_advanced_stock(&coord);
        assert_eq!(adv.standard_parts, 2.0);

        let raw = stockpile.stock.get(&coord).unwrap();
        assert_eq!(raw.ore, 6.0); // 10 - 4
        assert_eq!(raw.energy, 8.0); // 10 - 2
    }

    #[test]
    fn test_process_turn_production_chain_to_munitions() {
        let mut ind = IndustryGrid::default();
        let mut stockpile = TileStockpile::default();
        let op_grid = TileOperatingGrid::default();

        let coord = HexCoord::new(2, 3);
        ind.set_plant(
            coord,
            ManufacturingPlant {
                capacity: 1.0,
                active_recipe: Some(RecipeId::ProduceMunitions),
            },
        );

        // 原料＋部品
        stockpile
            .stock
            .insert(coord, ResourceAmounts::new(0.0, 5.0, 5.0, 0.0));
        ind.advanced_stocks.insert(
            coord,
            AdvancedStock {
                standard_parts: 1.0,
                ..default()
            },
        );

        ind.process_turn_production(&mut stockpile, &op_grid);

        let adv = ind.get_advanced_stock(&coord);
        assert_eq!(adv.munitions, 5.0);
        assert_eq!(adv.standard_parts, 0.0); // 1.0 消費
    }
}
