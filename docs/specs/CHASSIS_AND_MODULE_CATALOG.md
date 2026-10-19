# 初期モジュール・基本車台実数値カタログ仕様書

## (Turn 0 Initial Modules & Standard Chassis Data Catalog)

本ドキュメントは、本作（4X大戦略＋諸兵科連合戦闘団制＋サブタイル戦術）における**「ゲーム開始時（Turn 0）〜初期開拓期（Gen 1）において全国家が共通利用する基本車台（シャシー）、兵装モジュール、装甲・動力、および戦略輸送ドクトリン」**の実数値を定義したデータカタログ仕様書である。

---

## 1. カタログ策定の基本設計方針

### 1.1 特別な対エイリアン兵器を排した実利志向

- 初期人類勢力はエイリアン生態の詳細を把握しておらず、**「対エイリアン専用の奇策・特効兵器（火炎放射器等）は存在しない」**。
- 母星の軍事規格（ISO-Terran Baseline）に基づき開発された汎用火器（30C機関砲、80C低圧砲、同軸8C機銃、多目的ミサイル）の物理打撃力・破砕力によって対抗する。

### 1.2 戦略機動は「重トレーラー・鉄道」前提

- 戦闘車両自体の自走航続距離は戦術機動（親タイル内部・サブタイル戦闘空間）に最適化されている。
- 親タイル間を跨ぐ長距離の戦略機動（Strategic Mobility）は、**「道路上の大型重機トレーラー（HET: Heavy Equipment Transport）牽引」**または**「鉄道インフラ網（列車輸送）」**を前提とし、自走による履帯・足回り損耗や燃料浪費を抑止する。

### 1.3 同軸・副武装は「8C」規格統一

- 主砲同軸機銃、車長旋回銃架には、歩兵支援火器と完全に弾薬互換（8×60C弾）を持つ**「8C重機関銃（同軸8C）」**を採用し、小火器サプライチェーンを一本化する。

---

## 2. 3大基本車台（シャシー）実数値仕様

入植初期に現地工廠でライセンス・ノックダウン生産可能な3大車台。

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                          3大基本車台の性格・運用ドクトリン                  │
├─────────────────────────────────────────────────────────────────────────────┤
│  [Std-W6 6輪装輪]  : 自走高速・低燃費。舗装路特化。泥濘・岩盤でスタック。   │
│  [Std-T4 装軌履帯]  : 重装甲・全地形走破。自走長距離行軍で履帯寿命が激減。  │
│  [Std-L4 4脚多脚]  : 極限斜面・岩礁踏破。最高速が極めて鈍重、関節整備大。  │
└─────────────────────────────────────────────────────────────────────────────┘
```

| 車台型式 (ID) | 自重 (t) | 最大積載余力 (t) | 戦術基準速(km/h) | 整備負荷係数 | 戦略輸送適性(トレーラー/列車) | 地形走破係数（移動コスト倍率） |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| **Std-W6**汎用6輪装輪車台 | **5.0 t** | **7.0 t** | 75 | **0.80** | **◎ 自走・牽引両用**(平坦路なら自走転進可) | 舗装路: **0.7** / 平原: 1.0泥濘・湿地: **2.5** (スタック危険)岩盤急斜面: **3.0** / 瘴気: 1.2 |
| **Std-T4**中型装軌クローラー車台 | **9.0 t** | **15.0 t** | 45 | **1.20** | **○ トレーラー必須**(自走は舗装路を粉砕) | 舗装路: 1.0 / 平原: 1.0泥濘・湿地: **1.2** (安定走破)岩盤急斜面: **1.5** / 瘴気: 1.1 |
| **Std-L4**重作業用4脚多脚車台 | **7.0 t** | **9.0 t** | 28 | **1.60** | **△ 特殊低床車/列車**(全高が高く積載制約有) | 舗装路: 1.4 / 平原: 1.3 (鈍重)泥濘・湿地: **1.0** (埋没回避)岩盤急斜面: **0.85** (圧倒的走破)瘴気: 1.4 (関節防塵シール要) |

---

## 3. 初期兵装モジュール（Weapon Modules）実数値仕様

Turn 0時点で全国家が共通保有する実用兵装群。

| モジュールID | 兵装呼称 | 重量 (t) | 有効射程(サブタイル) | 威力区分 | 使用弾薬規格 | 特性・戦術的役割 |
| :--- | :--- | :---: | :---: | :--- | :--- | :--- |
| **WPN-01** | **30C共通機関砲**(*30C Autocannon*) | **1.4 t** | 1〜2 | 連射・運動エネルギー | **30×180C 弾薬箱**(1発約0.8kg) | 対軽装甲・対空・対エイリアン群れの標準兵装。高初速で硬質甲殻を削り取る。 |
| **WPN-02** | **80C低圧滑腔破砕砲**(*80C Low-Pressure Gun*) | **2.6 t** | 2〜3 | 面制圧・破砕榴弾 | **80C低圧一体薬筒**(1発約8.5kg) | 開拓用岩盤破砕砲の軍事転用。初速を抑えて反動を軽減。大質量弾頭で大型個体や建造物を粉砕。 |
| **WPN-03** | **同軸・旋回8C重機関銃**(*Coaxial 8C Machine Gun*) | **0.2 t** | 1 | 対人・近接阻止 | **8×60C 重小銃弾**(歩兵弾と共通) | 全車台の主砲同軸および車長ハッチに標準装備。歩兵・近接小型生物の掃射用。 |
| **WPN-04** | **4連装多目的誘導ミサイル**(*4-Cell Multi-Missile Pod*) | **1.1 t** | 2〜4 | 成形炸薬・精密打撃 | **120C多目的ロケット**(1発約35kg) | 重装甲目標・遠距離巨大性個体に対する切り札。瞬間投射力は高いが再装填ラグ大。 |

---

## 4. 装甲・動力・戦略輸送モジュール

### 4.1 防護装甲モジュール (Armor Modules)

- **ARM-01「基礎圧延均質鋼板 (RHA)」**:
  - 重量: 3.5t / 防護力: 100（標準）
  - 安価で溶接修理が容易。現地工廠の初期工作機械で即座に増産可能。
- **ARM-02「セラミック耐熱複合装甲」**:
  - 重量: 2.8t / 防護力: 160（高性能）
  - 軽量高剛性。成形炸薬や酸性体液に対して優れた耐性を持つが、修理に専用接着工数が必要。

### 4.2 動力パワーパック (Powerpack Modules)

- **PWR-01「水素燃料電池＋高トルクEV」**:
  - 重量: 2.2t / 出力: 450 kW / 静粛性ボーナス有
  - クリーンで高トルク。登坂力に優れるが、高純度水素補給インフラが必要。
- **PWR-02「多燃料対応改質ターボディーゼル」**:
  - 重量: 2.9t / 出力: 400 kW / 整備性抜群
  - 合成メタン、粗製炭化水素油、重油など現地調達可能なあらゆる可燃液体で稼働。

### 4.3 戦略輸送アセット（Strategic Mobility Assets）

- **TRN-01「60t積載 重機牽引セミトレーラー (HET)」**:
  - インフラ結合度 0.6 以上の舗装道路網において、戦闘団の装軌・多脚車両を積載して高速巡航（移動コスト 0.5倍）。
- **TRN-02「重軌道 装甲列車・フラットカー」**:
  - 敷設済み鉄道幹線上を走行。1編成で大隊規模（戦車64両）を燃料消費極小で長距離空輸並みの速度で輸送。

---

## 5. Bevy ECS 実装データ構造仕様 (Rust)

```rust
use bevy::prelude::*;

/// 基本車台（シャシー）コンポーネント
#[derive(Component, Debug, Clone, Reflect)]
pub struct VehicleChassis {
    pub chassis_id: String,
    pub chassis_type: ChassisType,
    pub dry_weight_tons: f32,
    pub max_payload_tons: f32,
    pub base_tactical_speed: u16,
    pub maintenance_factor: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ChassisType {
    Wheeled6x6,   // 装輪 Std-W6
    TrackedMid,   // 履帯 Std-T4
    QuadLegged,   // 多脚 Std-L4
}

/// 搭載兵装モジュールコンポーネント
#[derive(Component, Debug, Clone, Reflect)]
pub struct WeaponModuleSlot {
    pub main_weapon_id: String,
    pub main_caliber_c: u16,      // 30 (30C) または 80 (80C)
    pub coaxial_caliber_c: u16,   // 8 (8C固定)
    pub has_missile_pod: bool,    // 4連装ミサイル有無
    pub ammo_capacity_shots: u16, // 主砲即応弾数
}

/// 戦略機動・輸送状態コンポーネント
#[derive(Component, Debug, Clone, Reflect)]
pub struct StrategicMobilityStatus {
    pub transport_mode: TransportMode,
    pub loaded_on_transporter: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum TransportMode {
    SelfTacticalDrive, // 自走（戦術展開隊形）
    HetTrailerConvoy,  // 重機トレーラー車列
    RailroadArmoredTrain, // 鉄道輸送
}
```

---

## 6. ドキュメント相互参照

- [`../planning/MISSING_SPECS_EXECUTION_PLAN.md`](../planning/MISSING_SPECS_EXECUTION_PLAN.md) - 不足ドキュメント実行計画
- [`RESOURCE_ITEM_DICTIONARY.md`](./RESOURCE_ITEM_DICTIONARY.md) - 資源・弾薬物理データ辞書
- [`../units/force/concept_ground_vehicle_chassis_and_variants.md`](../units/force/concept_ground_vehicle_chassis_and_variants.md) - 陸上装甲車両総合仕様書
- [`../discussion/military_tactics/concept_starter_modules_and_common_chassis_catalog.md`](../discussion/military_tactics/concept_starter_modules_and_common_chassis_catalog.md) - Turn 0 初期カタログ検討書
