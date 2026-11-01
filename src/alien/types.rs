use bevy::prelude::*;
use std::collections::HashMap;

use crate::map::hex::HexCoord;

/// 巣窟のライフサイクル状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum NestLifecycle {
    /// 休眠状態: 人類への脅威度は最低、少数の小型種のみ生息
    #[default]
    Dormant,
    /// 活性状態: 瘴気放出開始、中型・大型種が警戒活動
    Active,
    /// 繁殖状態: 瘴気放出最大、急速な防衛生体生産
    Breeding,
    /// 暴走侵入状態（スウォーム）: 刺激限界突破、人類拠点への突進
    Swarming,
}

impl NestLifecycle {
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Dormant => "休眠中 (Dormant)",
            Self::Active => "活性化 (Active)",
            Self::Breeding => "繁殖期 (Breeding)",
            Self::Swarming => "暴走侵入 (Swarming)",
        }
    }

    /// 状態に応じた毎ターンの基準瘴気発生量
    pub fn base_miasma_emission(&self) -> f32 {
        match self {
            Self::Dormant => 0.05,
            Self::Active => 0.25,
            Self::Breeding => 0.50,
            Self::Swarming => 0.40,
        }
    }
}

/// エイリアンの巣窟コンポーネント
#[derive(Component, Debug, Clone, Reflect)]
pub struct AlienNest {
    pub coord: HexCoord,
    pub lifecycle: NestLifecycle,
    /// 現在の耐久度（最大 500.0）
    pub current_hp: f32,
    pub max_hp: f32,
    /// 人類による刺激ヘイト値 (0.0〜100.0)
    pub irritation: f32,
    /// 巣窟の固有識別子
    pub id: u32,
}

impl AlienNest {
    pub const MAX_IRRITATION: f32 = 100.0;
    pub const ACTIVE_THRESHOLD: f32 = 25.0;
    pub const BREEDING_THRESHOLD: f32 = 60.0;
    pub const SWARMING_THRESHOLD: f32 = 90.0;

    pub fn new(id: u32, coord: HexCoord) -> Self {
        Self {
            id,
            coord,
            lifecycle: NestLifecycle::Dormant,
            current_hp: 500.0,
            max_hp: 500.0,
            irritation: 0.0,
        }
    }

    /// 刺激ヘイト値を加算し、ライフサイクル状態を更新
    pub fn add_irritation(&mut self, amount: f32) {
        self.irritation = (self.irritation + amount).clamp(0.0, Self::MAX_IRRITATION);
        self.update_lifecycle_from_irritation();
    }

    /// 刺激ヘイト値に基づいてライフサイクルを遷移
    pub fn update_lifecycle_from_irritation(&mut self) {
        if self.irritation >= Self::SWARMING_THRESHOLD {
            self.lifecycle = NestLifecycle::Swarming;
        } else if self.irritation >= Self::BREEDING_THRESHOLD {
            self.lifecycle = NestLifecycle::Breeding;
        } else if self.irritation >= Self::ACTIVE_THRESHOLD {
            self.lifecycle = NestLifecycle::Active;
        } else {
            self.lifecycle = NestLifecycle::Dormant;
        }
    }
}

/// マップ上の各タイルの瘴気（Miasma）濃度を管理するリソース
#[derive(Resource, Default, Debug, Clone)]
pub struct MiasmaMap {
    /// 各タイルの瘴気濃度（0.0: なし 〜 1.0: 最大濃度）
    pub concentrations: HashMap<HexCoord, f32>,
}

impl MiasmaMap {
    pub const MAX_CONCENTRATION: f32 = 1.0;
    pub const DISSIPATION_RATE: f32 = 0.05; // 毎ターンの自然減衰率
    pub const DIFFUSION_COEFF: f32 = 0.15;  // 隣接タイルへの拡散係数

    #[inline]
    pub fn get(&self, coord: &HexCoord) -> f32 {
        self.concentrations.get(coord).copied().unwrap_or(0.0)
    }

    #[inline]
    pub fn set(&mut self, coord: HexCoord, value: f32) {
        let clamped = value.clamp(0.0, Self::MAX_CONCENTRATION);
        if clamped <= 0.001 {
            self.concentrations.remove(&coord);
        } else {
            self.concentrations.insert(coord, clamped);
        }
    }

    #[inline]
    pub fn add(&mut self, coord: HexCoord, amount: f32) {
        let current = self.get(&coord);
        self.set(coord, current + amount);
    }
}
