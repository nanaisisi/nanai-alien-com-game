use bevy::prelude::*;

use crate::faction::types::FactionId;
use crate::map::hex::HexCoord;

/// 戦闘団（Combat Group）のカテゴリー種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CombatGroupType {
    /// 偵察戦闘団: 高機動・視界確保特化
    Scout,
    /// 開拓/民間人戦闘団: 新たな拠点の設営能力
    Colonist,
    /// 軽歩兵戦闘団: 基本的な陸上戦闘部隊
    LightInfantry,
}

impl CombatGroupType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Scout => "偵察戦闘団 (Scout)",
            Self::Colonist => "開拓戦闘団 (Colonizer)",
            Self::LightInfantry => "軽歩兵戦闘団 (Light Infantry)",
        }
    }

    pub fn base_movement(&self) -> u32 {
        match self {
            Self::Scout => 3,
            Self::Colonist => 2,
            Self::LightInfantry => 2,
        }
    }

    pub fn base_max_hp(&self) -> u32 {
        match self {
            Self::Scout => 80,
            Self::Colonist => 50,
            Self::LightInfantry => 100,
        }
    }

    pub fn attack_power(&self) -> u32 {
        match self {
            Self::Scout => 15,
            Self::Colonist => 0,
            Self::LightInfantry => 30,
        }
    }

    pub fn attack_range(&self) -> u32 {
        match self {
            Self::Scout => 1,
            Self::Colonist => 0,
            Self::LightInfantry => 1,
        }
    }
}

/// ユニットの現在の行動・待機状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UnitActionState {
    /// 通常（待機または行動中）
    #[default]
    Idle,
    /// 防御態勢（継続ターン数に応じて防御力+25%〜+50%）
    Fortified { turns: u32 },
    /// 警戒監視中（敵が接近・視界内に入ると自動で目覚める）
    Alert,
    /// 休眠中（手動選択または敵接近まで自動巡回スキップ）
    Sleeping,
    /// 回復・修理中（毎ターンHP回復、全快でIdleへ自動復帰）
    Healing,
}

impl UnitActionState {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Idle => "待機/行動可能",
            Self::Fortified { turns } => {
                if *turns >= 2 {
                    "防御陣地展開 (強固 +50%)"
                } else {
                    "防御態勢 (+25%)"
                }
            }
            Self::Alert => "警戒監視 (Alert)",
            Self::Sleeping => "休眠待機 (Sleep)",
            Self::Healing => "回復・修理中 (Healing)",
        }
    }

    pub fn defense_multiplier(&self) -> f32 {
        match self {
            Self::Fortified { turns } => {
                if *turns >= 2 {
                    1.50
                } else {
                    1.25
                }
            }
            _ => 1.0,
        }
    }
}

/// ユニットの現在の配備・格納ステータス
#[derive(Component, Debug, Clone, PartialEq, Eq, Reflect, Default)]
pub enum DeploymentStatus {
    /// 地表アクティブ駐留 (大戦略マップに表示される主部隊: 1タイル最大1つ)
    #[default]
    ActiveGarrison,
    /// 地下要塞・シェルター待機 (非表示スタック: アクティブ部隊の予備)
    BunkerReserve { priority_order: u8 },
    /// 輸送艦・母艦・列車への積載中 (非表示スタック: ホスト追従)
    Embarked { carrier_entity: Entity },
    /// 港湾・基地でのモスボール休役保管 (非配備兵器: 出渠ラグあり)
    Mothballed { reactivation_turns_remaining: u8 },
    /// 工廠・デポ内の資材・予備役プール (実体ユニットではなく兵装ストック)
    DepotInventory,
}

impl DeploymentStatus {
    pub fn is_active_garrison(&self) -> bool {
        matches!(self, Self::ActiveGarrison)
    }

    pub fn is_hidden_stack(&self) -> bool {
        !self.is_active_garrison()
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ActiveGarrison => "アクティブ駐留 (地表露出)",
            Self::BunkerReserve { .. } => "地下要塞・シェルター待機 (非表示予備)",
            Self::Embarked { .. } => "輸送母艦/輸送機関に積載中",
            Self::Mothballed { .. } => "モスボール休役保管中 (港湾/基地)",
            Self::DepotInventory => "工廠/兵站デポ保管品",
        }
    }
}

/// 輸送母艦・格納施設コンポーネント (他のユニットを内部スタック可能)
#[derive(Component, Debug, Clone, Reflect)]
pub struct HangarBay {
    /// 最大積載スロット容量 (トン数または小隊数)
    pub capacity: u32,
    /// 現在内部に非表示格納されているユニットエンティティのリスト
    pub stored_units: Vec<Entity>,
}

impl Default for HangarBay {
    fn default() -> Self {
        Self {
            capacity: 4,
            stored_units: Vec::new(),
        }
    }
}

/// 国家・方面軍工廠の非配備兵器・艦艇プール
#[derive(Resource, Debug, Clone, Default)]
pub struct NationalArsenalPool {
    /// 勢力識別子
    pub faction_id: FactionId,
    /// 未配備・予備役の完成品車台ストック (シャシー名/ID -> 保管数)
    pub chassis_stock: std::collections::HashMap<String, u32>,
    /// 未配備の兵装モジュールストック (モジュール名/ID -> 保管数)
    pub weapon_module_stock: std::collections::HashMap<String, u32>,
    /// 母星遺産（Gen 0）の封印保管ストック
    pub legacy_gen0_sealed_units: Vec<String>,
}

/// 全体マップ上で活動する戦闘団ユニット
#[derive(Component, Debug, Clone, Reflect)]
pub struct Unit {
    pub faction: FactionId,
    pub group_type: CombatGroupType,
    pub coord: HexCoord,
    pub max_movement: u32,
    pub current_movement: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub is_exhausted: bool,
    pub action_state: UnitActionState,
    pub deployment_status: DeploymentStatus,
}

impl Unit {
    pub fn new(faction: FactionId, group_type: CombatGroupType, coord: HexCoord) -> Self {
        let max_movement = group_type.base_movement();
        let max_hp = group_type.base_max_hp();
        Self {
            faction,
            group_type,
            coord,
            max_movement,
            current_movement: max_movement,
            hp: max_hp,
            max_hp,
            is_exhausted: false,
            action_state: UnitActionState::Idle,
            deployment_status: DeploymentStatus::ActiveGarrison,
        }
    }

    pub fn reset_turn(&mut self) {
        self.current_movement = self.max_movement;
        self.is_exhausted = false;
    }
}

/// 現在選択されているユニットのエンティティ
#[derive(Resource, Default, Debug)]
pub struct SelectedUnit(pub Option<Entity>);

/// Mキーによるユニット移動指示モードの状態（後方互換）
#[derive(Resource, Default, Debug)]
pub struct MoveModeState(pub bool);

/// 右クリック長押しドラッグによる移動指示状態
#[derive(Resource, Default, Debug)]
pub struct RightDragMoveState {
    pub is_dragging: bool,
    pub start_coord: Option<HexCoord>,
    pub current_target: Option<HexCoord>,
}

/// ユニット移動可能タイルの表示用マーカー
#[derive(Component)]
pub struct MoveTargetMarker;

/// ユニット選択リング表示用マーカー
#[derive(Component)]
pub struct UnitSelectionRing;
