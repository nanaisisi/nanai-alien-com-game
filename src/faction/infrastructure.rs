use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::faction::territory::{FactionOutpost, TerritoryMap};
use crate::faction::types::FactionId;
use crate::map::MapGrid;
use crate::map::hex::HexCoord;

/// 動脈インフラの種別
/// (参照: docs/discussion/map_tile/concept_infrastructure_grid_and_transit_artery.md)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ArteryType {
    /// 舗装道路 / 高速道路網
    Road,
    /// 重軌道 / 貨物鉄道
    Rail,
    /// 基幹送電線 / 光ファイバー網
    PowerLine,
}

impl ArteryType {
    /// 動脈インフラの伝達効率係数
    pub fn efficiency_factor(&self) -> f32 {
        match self {
            ArteryType::Road => 0.85,
            ArteryType::Rail => 0.95,
            ArteryType::PowerLine => 0.90,
        }
    }
}

/// 2つの隣接ヘックス座標間の無向エッジ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ArteryEdge {
    pub a: HexCoord,
    pub b: HexCoord,
}

impl ArteryEdge {
    pub fn new(c1: HexCoord, c2: HexCoord) -> Self {
        // 正規化して順序不同にする
        if (c1.q, c1.r) <= (c2.q, c2.r) {
            Self { a: c1, b: c2 }
        } else {
            Self { a: c2, b: c1 }
        }
    }
}

/// マップ上の動脈インフラネットワーク管理リソース
#[derive(Resource, Default, Debug, Clone, Reflect)]
pub struct InfrastructureGrid {
    /// エッジごとの敷設インフラ
    pub edges: HashMap<ArteryEdge, HashSet<ArteryType>>,
    /// 各タイルの拠点（首都・前哨基地）からのインフラ結合度 (0.0 〜 1.0)
    pub connectivity: HashMap<HexCoord, f32>,
}

impl InfrastructureGrid {
    /// 2タイル間にインフラ動脈を追加
    pub fn add_artery(&mut self, c1: HexCoord, c2: HexCoord, artery: ArteryType) {
        let edge = ArteryEdge::new(c1, c2);
        self.edges.entry(edge).or_default().insert(artery);
    }

    /// 2タイル間の特定インフラが存在するか確認
    #[allow(dead_code)]
    pub fn has_artery(&self, c1: HexCoord, c2: HexCoord, artery: ArteryType) -> bool {
        let edge = ArteryEdge::new(c1, c2);
        self.edges
            .get(&edge)
            .is_some_and(|set| set.contains(&artery))
    }

    /// タイルごとの最高インフラ結合度を再計算
    /// 拠点（FactionOutpost）から自国領土内の接続エッジをBFSで探索し、結合度（減衰付き）を計算
    pub fn recalculate_connectivity(
        &mut self,
        outposts: &[(HexCoord, FactionId)],
        territory: &TerritoryMap,
        map_w: i32,
    ) {
        self.connectivity.clear();

        for &(outpost_coord, faction) in outposts {
            // 拠点は結合度 1.0
            self.connectivity.insert(outpost_coord, 1.0);

            let mut queue = VecDeque::new();
            queue.push_back((outpost_coord, 1.0f32));

            let mut visited = HashSet::new();
            visited.insert(outpost_coord);

            while let Some((curr, conn)) = queue.pop_front() {
                if conn <= 0.05 {
                    continue;
                }

                // 隣接タイルを探索
                for neighbor in curr.neighbors_with_width(map_w) {
                    // 自国領土外へは伝播しない（不可侵・主権原則）
                    if territory.get_owner(&neighbor) != Some(faction) {
                        continue;
                    }

                    let edge = ArteryEdge::new(curr, neighbor);
                    if let Some(arteries) = self.edges.get(&edge) {
                        // 接続されているインフラの中で最大の効率係数を採用
                        let max_factor = arteries
                            .iter()
                            .map(|a| a.efficiency_factor())
                            .fold(0.0f32, f32::max);

                        if max_factor > 0.0 {
                            let next_conn = conn * max_factor;
                            let current_best = self.connectivity.entry(neighbor).or_insert(0.0);
                            if next_conn > *current_best {
                                *current_best = next_conn;
                                if !visited.contains(&neighbor) {
                                    visited.insert(neighbor);
                                    queue.push_back((neighbor, next_conn));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// タイルのインフラ結合度を取得（存在しない場合は 0.0）
    pub fn get_connectivity(&self, coord: &HexCoord) -> f32 {
        self.connectivity.get(coord).copied().unwrap_or(0.0)
    }
}

/// ターン開始時にインフラ結合度を更新するシステム
pub fn update_infrastructure_connectivity(
    mut infra: ResMut<InfrastructureGrid>,
    outposts_query: Query<&FactionOutpost>,
    territory: Res<TerritoryMap>,
    map_grid: Res<MapGrid>,
) {
    let outposts: Vec<(HexCoord, FactionId)> = outposts_query
        .iter()
        .map(|outpost| (outpost.coord, outpost.faction))
        .collect();

    let map_w = if map_grid.width > 0 {
        map_grid.width
    } else {
        crate::map::settings::MapSize::default().width()
    };

    infra.recalculate_connectivity(&outposts, &territory, map_w);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artery_edge_normalization() {
        let c1 = HexCoord::new(2, 5);
        let c2 = HexCoord::new(3, 4);

        let e1 = ArteryEdge::new(c1, c2);
        let e2 = ArteryEdge::new(c2, c1);
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_infrastructure_connectivity_propagation() {
        let mut grid = InfrastructureGrid::default();
        let mut territory = TerritoryMap::default();

        let c0 = HexCoord::new(0, 0);
        let c1 = HexCoord::new(1, 0);
        let c2 = HexCoord::new(2, 0);

        territory.tile_owners.insert(c0, FactionId::Empire);
        territory.tile_owners.insert(c1, FactionId::Empire);
        territory.tile_owners.insert(c2, FactionId::Empire);

        // c0-c1 に鉄道、c1-c2 に道路を敷設
        grid.add_artery(c0, c1, ArteryType::Rail);
        grid.add_artery(c1, c2, ArteryType::Road);

        let outposts = vec![(c0, FactionId::Empire)];
        grid.recalculate_connectivity(&outposts, &territory, 50);

        assert_eq!(grid.get_connectivity(&c0), 1.0);
        // c1: 1.0 * 0.95 = 0.95
        assert!((grid.get_connectivity(&c1) - 0.95).abs() < 1e-4);
        // c2: 0.95 * 0.85 = 0.8075
        assert!((grid.get_connectivity(&c2) - 0.8075).abs() < 1e-4);
    }

    #[test]
    fn test_connectivity_blocked_by_territory() {
        let mut grid = InfrastructureGrid::default();
        let mut territory = TerritoryMap::default();

        let c0 = HexCoord::new(0, 0);
        let c1 = HexCoord::new(1, 0);

        territory.tile_owners.insert(c0, FactionId::Empire);
        // c1 は他国領土
        territory.tile_owners.insert(c1, FactionId::GrandDuchy);

        grid.add_artery(c0, c1, ArteryType::Rail);

        let outposts = vec![(c0, FactionId::Empire)];
        grid.recalculate_connectivity(&outposts, &territory, 50);

        assert_eq!(grid.get_connectivity(&c0), 1.0);
        // 他国領土へは伝播しない
        assert_eq!(grid.get_connectivity(&c1), 0.0);
    }
}
