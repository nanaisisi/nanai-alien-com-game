use bevy::prelude::*;

/// 水域タイルの塩分濃度区分 (実用塩分単位 PSU / ‰)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect, Default)]
#[reflect(Component)]
pub enum WaterSalinity {
    /// 淡水 (Freshwater): < 0.5‰
    /// 一般の内陸湖沼・河川。農業・生活用水の水源、豊富な淡水魚資源
    #[default]
    Fresh,

    /// 汽水 (Brackish): 0.5‰ 〜 30‰
    /// カスピ海（約12‰）やバルト海のような巨大内陸海・河口域。豊かな魚介類と緩やかな水力
    Brackish,

    /// 海水 (Saline / Marine): 30‰ 〜 50‰
    /// 外洋（海洋平均 約35‰）。潮汐・海流エネルギー、海洋航行
    Saline,

    /// 高塩分 / 塩湖 (Hypersaline): > 50‰
    /// 死海（約300‰）や乾燥地帯の内陸塩湖。魚類生息は困難だが、蒸発岩・リチウム・塩類鉱物（生産力）に富む
    Hypersaline,
}

impl WaterSalinity {
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Fresh => "淡水 (Freshwater / <0.5‰)",
            Self::Brackish => "汽水 (Brackish / 0.5-30‰)",
            Self::Saline => "海水 (Saline / ~35‰)",
            Self::Hypersaline => "高塩分・塩湖 (Hypersaline / >50‰)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Fresh => "純度の高い淡水域。生活用水・灌漑に適し、水産バイオマスが極めて豊富。",
            Self::Brackish => "カスピ海型内陸水域。淡水と塩分が混じり合い特有の魚介生態系と水力エネルギーを保持。",
            Self::Saline => "典型的な海洋水。豊富な海流エネルギーと広域航路を提供する標準的塩分濃度。",
            Self::Hypersaline => "死海・グレートソルト湖型。強い塩分により生物生息は限定的だが、晶出鉱物・蒸発塩（生産力）が豊富。",
        }
    }

    /// 塩分濃度に応じた基礎水域産出力
    pub fn base_output(&self) -> super::resource::ResourceAmounts {
        use super::resource::ResourceAmounts as R;
        match self {
            Self::Fresh => R::new(2.0, 0.0, 0.0, 0.0),      // 淡水魚・水源（食料2）
            Self::Brackish => R::new(1.5, 0.0, 0.5, 0.0),   // 汽水水産（食料1.5）+ 潮汐水流（エネルギー0.5）
            Self::Saline => R::new(1.0, 0.0, 1.0, 0.0),     // 海洋漁獲（食料1）+ 海流エネルギー（エネルギー1）
            Self::Hypersaline => R::new(0.0, 2.0, 0.0, 0.0),// 塩・鉱物・リチウム晶出（生産2）
        }
    }

    /// ホバー・ベースカラーの微細な色調調整
    pub fn color_tint(&self) -> Color {
        match self {
            Self::Fresh => Color::srgb(0.14, 0.40, 0.50),      // 澄んだ青緑・アクア
            Self::Brackish => Color::srgb(0.12, 0.32, 0.48),   // カスピ海調の深緑蒼色
            Self::Saline => Color::srgb(0.10, 0.25, 0.48),     // 外洋ディープブルー
            Self::Hypersaline => Color::srgb(0.20, 0.45, 0.46),// 塩湖・ターコイズ＆塩結晶ホワイト寄り
        }
    }
}

/// タイルの地形種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TerrainType {
    /// 平原: 標準的な地形、視界良好、移動コスト低
    Plains,
    /// 丘陵: 視界ボーナス、防御有利
    Hills,
    /// 森林/原生林: 遮蔽あり、機甲部隊は進行しづらい
    Forest,
    /// 山岳: 通行不能または極めて高コスト、航空機のみ通過可
    Mountains,
    /// 海洋/水域: 陸上部隊進入不可（艦船・航空機のみ）
    Ocean,
    /// 湖沼/内陸淡水: 閉鎖水域。食料資源豊富、都市の水源
    Lake,
    /// 毒性湿地/瘴気帯: エイリアン活動度高、人類部隊に継続ダメージやデバフ
    ToxicSwamp,
    /// 密林/熱帯雨林: 鬱蒼とした巨大樹海。視界遮蔽が極めて高く機甲部隊の移動が著しく困難。バイオマスと未知の生態系研究に適する
    Jungle,
}

impl TerrainType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Plains => "平原 (Plains)",
            Self::Hills => "丘陵 (Hills)",
            Self::Forest => "温帯林 (Forest)",
            Self::Mountains => "険峻な山岳 (Mountains)",
            Self::Ocean => "海洋 (Ocean)",
            Self::Lake => "湖沼 (Lake)",
            Self::ToxicSwamp => "瘴気湿地 (Toxic Swamp)",
            Self::Jungle => "熱帯樹海 (Jungle)",
        }
    }

    /// 地形の基礎色 (SF感のあるスタイリッシュなパレット)
    pub fn base_color(&self) -> Color {
        match self {
            Self::Plains => Color::srgb(0.24, 0.42, 0.28), // 深い緑
            Self::Hills => Color::srgb(0.48, 0.45, 0.35),  // 黄土・岩色
            Self::Forest => Color::srgb(0.12, 0.30, 0.22), // 濃緑
            Self::Mountains => Color::srgb(0.38, 0.40, 0.46), // 暗めの岩石色
            Self::Ocean => Color::srgb(0.10, 0.25, 0.48),  // ディープブルー（外洋）
            Self::Lake => Color::srgb(0.14, 0.38, 0.48),   // 透き通る淡水・アクアブルー（湖沼）
            Self::ToxicSwamp => Color::srgb(0.35, 0.18, 0.45), // 瘴気パープル
            Self::Jungle => Color::srgb(0.06, 0.24, 0.14), // 深緑・密林グリーン
        }
    }

    /// ホバー時の強調色
    pub fn hovered_color(&self) -> Color {
        match self {
            Self::Plains => Color::srgb(0.38, 0.60, 0.42),
            Self::Hills => Color::srgb(0.65, 0.62, 0.50),
            Self::Forest => Color::srgb(0.24, 0.48, 0.35),
            Self::Mountains => Color::srgb(0.55, 0.58, 0.65),
            Self::Ocean => Color::srgb(0.22, 0.42, 0.70),
            Self::Lake => Color::srgb(0.25, 0.55, 0.68),
            Self::ToxicSwamp => Color::srgb(0.52, 0.30, 0.65),
            Self::Jungle => Color::srgb(0.15, 0.42, 0.25),
        }
    }

    /// 3D描画時の高さオフセット（起伏表現）
    pub fn height(&self) -> f32 {
        match self {
            Self::Ocean | Self::Lake => 0.1,
            Self::ToxicSwamp => 0.15,
            Self::Plains => 0.25,
            Self::Forest => 0.35,
            Self::Jungle => 0.42, // 巨木の林冠によりForestより一段高い
            Self::Hills => 0.55,
            Self::Mountains => 0.95,
        }
    }

    /// 移動コスト（1.0 = 標準）
    pub fn movement_cost(&self) -> f32 {
        match self {
            Self::Plains => 1.0,
            Self::Hills => 1.5,
            Self::Forest => 1.8,
            Self::ToxicSwamp => 2.0,
            Self::Jungle => 2.5, // 密林・巨木・つる植物により通行困難
            Self::Mountains | Self::Ocean | Self::Lake => 999.0, // 通行不可（水域・険山）
        }
    }

    /// 進入可能かどうか（一般地上部隊）
    pub fn is_passable_ground(&self) -> bool {
        !matches!(self, Self::Mountains | Self::Ocean | Self::Lake)
    }

    /// 地形の基礎産出力。内部値は浮動小数点 (食料=Biomass, 生産=構造鉱物, エネルギー, 科学)
    pub fn base_output(&self) -> super::resource::ResourceAmounts {
        use super::resource::ResourceAmounts as R;
        // R::new(食料, 生産, エネルギー, 科学)
        match self {
            Self::Plains => R::new(2.0, 1.0, 0.0, 0.0),    // 食料豊富
            Self::Hills => R::new(0.0, 2.0, 1.0, 0.0),     // 生産力高
            Self::Forest => R::new(1.0, 2.0, 0.0, 0.0),    // 食料+生産
            Self::Mountains => R::new(0.0, 0.0, 0.0, 1.0), // 科学観測
            Self::Ocean => R::new(1.0, 0.0, 1.0, 0.0),     // 食料+エネルギー（海流/潮汐）
            Self::Lake => R::new(2.0, 0.0, 0.0, 0.0),      // 淡水資源・豊かな生態系（食料+1）
            Self::ToxicSwamp => R::new(0.0, 0.0, 2.0, 1.0), // 瘴気エネルギー+研究
            Self::Jungle => R::new(2.0, 1.0, 0.0, 2.0),     // 圧倒的バイオマス(食料2) + 未知ゲノム・生態系(科学2)
        }
    }

    /// 塩分濃度を考慮した産出力（水域タイルの場合、塩分特性が反映される）
    pub fn output_with_salinity(&self, salinity: Option<WaterSalinity>) -> super::resource::ResourceAmounts {
        if matches!(self, Self::Ocean | Self::Lake) {
            if let Some(sal) = salinity {
                return sal.base_output();
            }
        }
        self.base_output()
    }

    /// 塩分濃度を考慮した基礎色
    pub fn base_color_with_salinity(&self, salinity: Option<WaterSalinity>) -> Color {
        if matches!(self, Self::Ocean | Self::Lake) {
            if let Some(sal) = salinity {
                return sal.color_tint();
            }
        }
        self.base_color()
    }
}
