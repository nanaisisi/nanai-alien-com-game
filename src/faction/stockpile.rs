use std::collections::HashMap;

use bevy::prelude::*;

use super::territory::TerritoryMap;
use super::types::{FactionId, FactionResources, PlayerFaction};
use crate::map::MapGrid;
use crate::map::hex::HexCoord;
use crate::map::resource::ResourceAmounts;
use crate::state::AppState;
use crate::turn::AdvanceTurnMessage;

/// 領土タイルごとの在庫。勢力の合計は実体を持たず、ここから派生させる。
#[derive(Resource, Default, Debug)]
pub struct TileStockpile {
    pub stock: HashMap<HexCoord, ResourceAmounts>,
}

impl TileStockpile {
    /// 指定勢力が所有するタイル在庫の合計（派生値）
    pub fn total_for(&self, faction: FactionId, territory: &TerritoryMap) -> ResourceAmounts {
        self.stock
            .iter()
            .filter(|(coord, _)| territory.get_owner(coord) == Some(faction))
            .fold(ResourceAmounts::default(), |acc, (_, a)| acc + *a)
    }

    /// 指定勢力が所有する領土タイルの毎ターン産出合計（派生値）
    pub fn per_turn_for(
        &self,
        faction: FactionId,
        territory: &TerritoryMap,
        map_grid: &MapGrid,
    ) -> ResourceAmounts {
        territory
            .tile_owners
            .iter()
            .filter(|&(_, owner)| *owner == faction)
            .filter_map(|(coord, _)| map_grid.terrain_data.get(coord))
            .fold(ResourceAmounts::default(), |acc, terrain| {
                acc + terrain.base_output()
            })
    }
}

/// 初期領土生成後に、各派閥の初期拠点タイルに初期在庫を配備
pub fn initialize_outpost_stockpile(
    mut stockpile: ResMut<TileStockpile>,
    query: Query<&crate::faction::territory::FactionOutpost>,
) {
    if !stockpile.stock.is_empty() {
        return;
    }

    for outpost in &query {
        // 各派閥の初期備蓄 (食料: 80, 生産: 50, エネルギー: 120, 科学: 30)
        let initial_stock = ResourceAmounts::new(80.0, 50.0, 120.0, 30.0);
        stockpile.stock.insert(outpost.coord, initial_stock);
    }
}

/// ターン進行時に、領有タイルの基礎産出をそのタイルの在庫へ加算する
pub fn accrue_tile_output(
    mut messages: MessageReader<AdvanceTurnMessage>,
    map_grid: Res<MapGrid>,
    territory: Res<TerritoryMap>,
    mut stockpile: ResMut<TileStockpile>,
) {
    for _ in messages.read() {
        for coord in territory.tile_owners.keys() {
            if let Some(terrain) = map_grid.terrain_data.get(coord) {
                *stockpile.stock.entry(*coord).or_default() += terrain.base_output();
            }
        }
    }
}

/// タイル在庫の合計値および領土タイルの毎ターン産出を FactionResources に同期するシステム
pub fn sync_stockpile_to_faction_resources(
    player_faction: Res<PlayerFaction>,
    stockpile: Res<TileStockpile>,
    territory: Res<TerritoryMap>,
    map_grid: Res<MapGrid>,
    mut resources: ResMut<FactionResources>,
) {
    let faction = player_faction.0;

    // 毎ターン産出合計
    let per_turn = stockpile.per_turn_for(faction, &territory, &map_grid);
    let (food_pt, prod_pt, energy_pt, sci_pt) = per_turn.rounded();
    resources.food_per_turn = food_pt;
    resources.production_per_turn = prod_pt;
    resources.energy_per_turn = energy_pt;
    resources.science_per_turn = sci_pt;

    // タイル在庫の合計値（派生値）
    let total = stockpile.total_for(faction, &territory);
    let (food_tot, prod_tot, energy_tot, sci_tot) = total.rounded();
    resources.food = food_tot;
    resources.production = prod_tot;
    resources.energy = energy_tot;
    resources.science = sci_tot;
}

/// 在庫をクリア（タイトルへ戻った時など）
pub fn reset_tile_stockpile(mut stockpile: ResMut<TileStockpile>) {
    stockpile.stock.clear();
}

pub struct StockpilePlugin;

impl Plugin for StockpilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileStockpile>()
            .add_systems(
                Update,
                (
                    initialize_outpost_stockpile.run_if(in_state(AppState::InGame)),
                    accrue_tile_output.run_if(in_state(AppState::InGame)),
                    sync_stockpile_to_faction_resources
                        .after(accrue_tile_output)
                        .run_if(in_state(AppState::InGame)),
                ),
            )
            .add_systems(OnEnter(AppState::Title), reset_tile_stockpile);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_is_derived_from_owned_tiles() {
        let a = HexCoord::new(0, 0);
        let b = HexCoord::new(1, 0);
        let mut territory = TerritoryMap::default();
        territory.tile_owners.insert(a, FactionId::Empire);
        territory.tile_owners.insert(b, FactionId::GrandDuchy);

        let mut sp = TileStockpile::default();
        sp.stock.insert(a, ResourceAmounts::new(2.0, 1.0, 0.0, 0.0));
        sp.stock.insert(b, ResourceAmounts::new(9.0, 9.0, 9.0, 9.0));

        let total = sp.total_for(FactionId::Empire, &territory);
        assert_eq!(total.rounded(), (2, 1, 0, 0));
    }
}
