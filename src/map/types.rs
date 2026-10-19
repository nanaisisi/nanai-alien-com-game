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

/// 河川の規模・流量階層 (River Scale / Flow Volume)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Default)]
pub enum RiverScale {
    /// 支流・細流 (Minor Stream): 山岳・高地の水源付近。歩兵や車両は軽微なコストで渡河可能
    #[default]
    Minor,
    /// 主流・中河川 (Major River): 支流の合流後。渡河に1ターンまたは移動力の大部分を消費
    Major,
    /// 大河・国際河川 (Great River): 流域下流部・河口付近。架橋工兵や水陸両用・跳躍兵装なしの渡河は極めて困難
    Great,
}

impl RiverScale {
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Minor => "支流・小河川 (Minor River)",
            Self::Major => "本流・中河川 (Major River)",
            Self::Great => "大河・国際河川 (Great River)",
        }
    }

    /// 渡河時の追加移動力コスト（平時地上ユニット）
    pub fn movement_penalty(&self) -> u32 {
        match self {
            Self::Minor => 1,
            Self::Major => 2,
            Self::Great => 3,
        }
    }

    /// 渡河攻撃時の攻撃力減衰率 (0.00 〜 1.00)
    pub fn combat_penalty_mult(&self) -> f32 {
        match self {
            Self::Minor => 0.85, // -15%
            Self::Major => 0.65, // -35%
            Self::Great => 0.50, // -50%
        }
    }

    /// 3Dメッシュ描画時の河川ライン幅
    pub fn visual_width(&self) -> f32 {
        match self {
            Self::Minor => 0.12,
            Self::Major => 0.20,
            Self::Great => 0.32,
        }
    }
}

/// マップ全体の状態を保持するリソース
#[derive(Resource, Default)]
pub struct MapGrid {
    pub tiles: HashMap<HexCoord, Entity>,
    pub terrain_data: HashMap<HexCoord, TerrainType>,
    pub water_salinity: HashMap<HexCoord, WaterSalinity>,
    /// 河川が存在するエッジとその規模階層一覧
    pub rivers: HashMap<HexEdge, RiverScale>,
    /// 開削された運河（地峡運河・内陸航路）が存在するエッジ一覧
    pub canals: HashSet<HexEdge>,
    pub width: i32,
    pub height: i32,
}

impl MapGrid {
    /// 指定タイルが河川に面しているか判定
    pub fn has_river(&self, coord: &HexCoord) -> bool {
        let w = if self.width > 0 { self.width } else { crate::map::hex::MAP_WIDTH };
        for neighbor in coord.neighbors_with_width(w) {
            let edge = HexEdge::new(*coord, neighbor);
            if self.rivers.contains_key(&edge) {
                return true;
            }
        }
        false
    }

    /// 2つの隣接タイルの境界に河川が流れているか判定
    pub fn has_river_between(&self, a: &HexCoord, b: &HexCoord) -> bool {
        let edge = HexEdge::new(*a, *b);
        self.rivers.contains_key(&edge)
    }

    /// 2つの隣接タイルの境界における河川規模を取得
    pub fn get_river_between(&self, a: &HexCoord, b: &HexCoord) -> Option<RiverScale> {
        let edge = HexEdge::new(*a, *b);
        self.rivers.get(&edge).copied()
    }

    /// 指定タイルが運河に面しているか判定
    pub fn has_canal(&self, coord: &HexCoord) -> bool {
        let w = if self.width > 0 { self.width } else { crate::map::hex::MAP_WIDTH };
        for neighbor in coord.neighbors_with_width(w) {
            let edge = HexEdge::new(*coord, neighbor);
            if self.canals.contains(&edge) {
                return true;
            }
        }
        false
    }

    /// 2つの隣接タイルの境界に運河が存在するか判定
    pub fn has_canal_between(&self, a: &HexCoord, b: &HexCoord) -> bool {
        let edge = HexEdge::new(*a, *b);
        self.canals.contains(&edge)
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

/// 運河オーバーレイマーカー
#[derive(Component)]
pub struct CanalOverlayLine;

/// タイル産出（Yield）オーバーレイマーカー
#[derive(Component)]
pub struct YieldOverlayTag;

