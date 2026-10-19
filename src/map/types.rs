use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use super::hex::HexCoord;
use super::terrain::{TerrainType, WaterSalinity};

pub const HEX_RADIUS: f32 = 1.0;

/// ヘックスの辺（Edge）の正規化識別子（隣接する2つのHexCoordペア）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HexEdge {
    pub tile_a: HexCoord,
    pub tile_b: HexCoord,
}

impl HexEdge {
    pub fn new(a: HexCoord, b: HexCoord) -> Self {
        if (a.q, a.r) <= (b.q, b.r) {
            Self { tile_a: a, tile_b: b }
        } else {
            Self { tile_a: b, tile_b: a }
        }
    }
}

/// マップ全体の状態を保持するリソース
#[derive(Resource, Default)]
pub struct MapGrid {
    pub tiles: HashMap<HexCoord, Entity>,
    pub terrain_data: HashMap<HexCoord, TerrainType>,
    pub water_salinity: HashMap<HexCoord, WaterSalinity>,
    /// 河川が存在するエッジ一覧
    pub rivers: HashSet<HexEdge>,
    pub width: i32,
    pub height: i32,
}

impl MapGrid {
    /// 指定タイルが河川に面しているか判定
    pub fn has_river(&self, coord: &HexCoord) -> bool {
        let w = if self.width > 0 { self.width } else { crate::map::hex::MAP_WIDTH };
        for neighbor in coord.neighbors_with_width(w) {
            let edge = HexEdge::new(*coord, neighbor);
            if self.rivers.contains(&edge) {
                return true;
            }
        }
        false
    }

    /// 2つの隣接タイルの境界に河川が流れているか判定
    pub fn has_river_between(&self, a: &HexCoord, b: &HexCoord) -> bool {
        let edge = HexEdge::new(*a, *b);
        self.rivers.contains(&edge)
    }
}

/// マップ表示設定（Civ6スタイルのグリッド表示 G、産出アイコン表示 Y）
#[derive(Resource, Debug, Clone)]
pub struct MapDisplaySettings {
    pub show_grid: bool,
    pub show_yields: bool,
}

impl Default for MapDisplaySettings {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_yields: true,
        }
    }
}

#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub terrain: TerrainType,
    pub salinity: Option<WaterSalinity>,
}

#[derive(Component)]
pub struct MapRoot;

/// グリッド線オーバーレイマーカー
#[derive(Component)]
pub struct GridOverlayLine;

/// 河川オーバーレイマーカー
#[derive(Component)]
pub struct RiverOverlayLine;

/// タイル産出（Yield）オーバーレイマーカー
#[derive(Component)]
pub struct YieldOverlayTag;
