use std::collections::HashMap;
use bevy::prelude::*;

use crate::faction::infrastructure::InfrastructureGrid;
use crate::faction::stockpile::TileStockpile;
use crate::faction::territory::TerritoryMap;
use crate::map::hex::HexCoord;
use crate::map::resource::ResourceAmounts;

/// タイル維持費設定コンポーネント / データ
/// (参照: docs/planning/MASTER_IMPLEMENTATION_PLAN.md - Phase 1)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct TileUpkeep {
    /// 毎ターン消費する維持リソース（主にエネルギーや部品）
    pub cost: ResourceAmounts,
}

impl Default for TileUpkeep {
    fn default() -> Self {
        Self {
            // 基本的な開発タイルの維持費: エネルギー 1.0
            cost: ResourceAmounts::new(0.0, 0.0, 1.0, 0.0),
        }
    }
}

/// ターン進行時に各タイルの維持費徴収と稼働判定を行うシステム
pub fn process_turn_upkeep_system(
    mut messages: MessageReader<crate::turn::AdvanceTurnMessage>,
    territory: Res<TerritoryMap>,
    infra: Res<InfrastructureGrid>,
    mut stockpile: ResMut<TileStockpile>,
    mut operating_grid: ResMut<TileOperatingGrid>,
) {
    for _ in messages.read() {
        operating_grid.process_turn_upkeep(&territory, &mut stockpile, &infra);
    }
}

/// タイル稼働状況コンポーネント / データ
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct TileOperatingState {
    /// 稼働率 (0.0 〜 1.0)。産出や加工効率に直接乗算される
    pub efficiency: f32,
    /// 補給・維持費不足またはインフラ断絶による負荷状態
    pub is_strained: bool,
}

impl Default for TileOperatingState {
    fn default() -> Self {
        Self {
            efficiency: 1.0,
            is_strained: false,
        }
    }
}

/// 全タイルの維持費と稼働状態を統括するリソース
#[derive(Resource, Default, Debug, Clone)]
pub struct TileOperatingGrid {
    pub upkeeps: HashMap<HexCoord, TileUpkeep>,
    pub states: HashMap<HexCoord, TileOperatingState>,
}

impl TileOperatingGrid {
    /// タイルの維持費を設定
    pub fn set_upkeep(&mut self, coord: HexCoord, upkeep: TileUpkeep) {
        self.upkeeps.insert(coord, upkeep);
    }

    /// タイルの稼働状態を取得
    pub fn get_state(&self, coord: &HexCoord) -> TileOperatingState {
        self.states.get(coord).copied().unwrap_or_default()
    }

    /// ターン更新時に各タイルの維持費を徴収し、稼働率を判定
    /// - タイル在庫に十分な資源がある場合: 維持費を消費し、インフラ結合度に基づいて稼働率を決定
    /// - 資源が不足している場合: 稼働率が半減またはゼロに低下し、is_strained フラグが true になる
    pub fn process_turn_upkeep(
        &mut self,
        territory: &TerritoryMap,
        stockpile: &mut TileStockpile,
        infra: &InfrastructureGrid,
    ) {
        for (&coord, _faction) in &territory.tile_owners {
            let upkeep = self.upkeeps.get(&coord).copied().unwrap_or_default();
            let conn = infra.get_connectivity(&coord);

            let stock = stockpile.stock.entry(coord).or_default();

            // 維持費支払いの判定（エネルギーと鉱物が足りているか）
            let can_afford = stock.energy >= upkeep.cost.energy
                && stock.ore >= upkeep.cost.ore
                && stock.biomass >= upkeep.cost.biomass;

            let (efficiency, is_strained) = if can_afford {
                stock.energy -= upkeep.cost.energy;
                stock.ore -= upkeep.cost.ore;
                stock.biomass -= upkeep.cost.biomass;

                // インフラ結合度に応じた稼働率（最低でも 0.50 は保証、結合度 1.0 で 1.0）
                // 結合度が 0 の場合はインフラ未結合ペナルティ
                let eff = (0.5 + 0.5 * conn).clamp(0.1, 1.0);
                let strained = conn < 0.2;
                (eff, strained)
            } else {
                // 資源不足時は一部のみ消費可能な分だけ引く（または 0 にする）
                stock.energy = (stock.energy - upkeep.cost.energy).max(0.0);
                stock.ore = (stock.ore - upkeep.cost.ore).max(0.0);
                stock.biomass = (stock.biomass - upkeep.cost.biomass).max(0.0);

                // 稼働率は 0.2 に低下し、過負荷・機能不全状態
                (0.2, true)
            };

            self.states.insert(
                coord,
                TileOperatingState {
                    efficiency,
                    is_strained,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::faction::types::FactionId;

    #[test]
    fn test_process_turn_upkeep_full_supply() {
        let mut grid = TileOperatingGrid::default();
        let mut territory = TerritoryMap::default();
        let mut stockpile = TileStockpile::default();
        let mut infra = InfrastructureGrid::default();

        let coord = HexCoord::new(1, 1);
        territory.tile_owners.insert(coord, FactionId::Empire);

        // 十分な在庫
        stockpile.stock.insert(coord, ResourceAmounts::new(10.0, 10.0, 10.0, 10.0));
        // インフラ結合度 1.0
        infra.connectivity.insert(coord, 1.0);

        grid.set_upkeep(coord, TileUpkeep {
            cost: ResourceAmounts::new(0.0, 0.0, 2.0, 0.0),
        });

        grid.process_turn_upkeep(&territory, &mut stockpile, &infra);

        let stock = stockpile.stock.get(&coord).unwrap();
        assert_eq!(stock.energy, 8.0);

        let state = grid.get_state(&coord);
        assert_eq!(state.efficiency, 1.0);
        assert!(!state.is_strained);
    }

    #[test]
    fn test_process_turn_upkeep_insufficient_supply() {
        let mut grid = TileOperatingGrid::default();
        let mut territory = TerritoryMap::default();
        let mut stockpile = TileStockpile::default();
        let infra = InfrastructureGrid::default();

        let coord = HexCoord::new(1, 1);
        territory.tile_owners.insert(coord, FactionId::Empire);

        // エネルギー在庫が不足
        stockpile.stock.insert(coord, ResourceAmounts::new(10.0, 10.0, 0.5, 10.0));

        grid.set_upkeep(coord, TileUpkeep {
            cost: ResourceAmounts::new(0.0, 0.0, 2.0, 0.0),
        });

        grid.process_turn_upkeep(&territory, &mut stockpile, &infra);

        let state = grid.get_state(&coord);
        assert_eq!(state.efficiency, 0.2);
        assert!(state.is_strained);
    }
}
