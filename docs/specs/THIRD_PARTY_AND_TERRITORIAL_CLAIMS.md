# 自発的失踪者・軍閥（第三極）接触・調達・領土要求仕様書

## (Frontier Dissidents, Black Market Procurement & Territorial Claims Specification)

本ドキュメントは、本作（4X大戦略＋諸兵科連合戦闘団制＋脱都市タイル進展）における**「非国家第三極（自発的失踪者・脱退コミュニティ）との接触と軍事機密防護」「闇市調達メカニクス」「自然国境に基づく4段階領土主張（Territorial Claims）と開戦事由（Casus Belli）生成」**を定義した具象仕様書である。

---

## 1. コア設計思想：「非対称な接触と、守るべき軍事機密」

### 1.1 自発的失踪者（Frontier Dissidents）の性格

- 国家の過酷な開拓ノルマや統制から逃亡し、公的台帳から失踪した元市民・開拓民・脱走兵が辺境・緩衝地帯に形成した集落群。
- 正規の軍需工場を持たないが、**「民生用重機・クローラーの現地装甲化」「発破用火薬・削岩レーザーの兵器転用」**など、過酷な原生環境を生き抜く中で培われた泥臭く実用的な武装（テクニカル兵器）を持つ。

### 1.2 軍事機密防護の原則（Security Compartmentalization）

- 国家が失踪者集落を取り込み（帰順）または闇市取引を行う際、**「自国の軍事機密（最新C規格弾薬の配合、暗号キー、MIL-SPEC工廠データ、虎の子Gen 0技術）は厳格に隔離・防護」**される。
- 失踪者に供与・取引されるのは「旧式民生品・汎用資材（食料、医薬品、基礎部品、旧式8C実包）」に限定され、国家中枢の軍事機密が第三極や他国スパイへ漏洩しないフェイルセーフ構造を持つ。

---

## 2. 失踪者コミュニティとの接触・インタラクション仕様

調査軍団や軽歩兵軍団が未踏タイルへ進入した際、確率で失踪者集落と遭遇する。

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 失踪者集落遭遇時の3大アプローチ（意思決定）                 │
├─────────────────────────────────────────────────────────────────────────────┤
│  [1. 平和的帰順・自治区編入 (Integration)] ── 医薬・食料支援と恩赦の付与   │
│      成果: サブタイルの人口・労働力即時編入、初期インフラ無償獲得           │
│      防護: 軍事機密は遮断、準市民権・地方警察レベルの自治組織にとどめる    │
│                                                                             │
│  [2. 闇市・非公式物資調達 (Black Market Deal)] ── 密輸ルートの開設         │
│      成果: C規格外の独自現地モジュール、異星生体組織、他国スクラップ調達   │
│      代償: CR資金消費、国内規律・防諜リスク微増                             │
│                                                                             │
│  [3. 武力制圧・強制退去 (Military Eviction)] ── 治安部隊による実力排除     │
│      成果: 資源タイルの完全独占・自由開発権                                 │
│      代償: 自国出身者が多い場合、国内支持率低下・兵士士気ペナルティ         │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.1 闇市調達（Black Market Procurement）カタログ

正規サプライチェーンを介さず、失踪者闇市から調達可能な特殊アセット。

| 調達品目 | 費用 (CR / 物資) | 性能・効果 | 機密保全レベル |
| :--- | :---: | :--- | :---: |
| **現地改修型装甲重機**(*Field-Mod Technical*) | 800 CR | Std-W6準拠の急造装甲バギー。工廠ラインを使わず即座に補充可能。 | **非機密 (民生品)** |
| **粗製発破成形炸薬**(*Crude Blasting Charges*) | 基礎部品 5t | 1回限りの大威力陣地破壊弾。命中精度は劣るが要塞・岩盤に特効。 | **非機密 (発破材)** |
| **未精製 異星生体試料**(*Raw Xeno-Biomass*) | 1,200 CR | 巣窟深部から採取された高純度原質。バイオ研究速度 +20%（3T）。 | **厳重隔離** |
| **他国通信傍受パッチ**(*Scavenged Signal Key*) | 機能鉱物 2t | 隣国の作戦ベクトル・移動目標を今後5ターンの間マップ上に表示。 | **対破壊暗号化** |

---

## 3. 自然国境に基づく4段階領土主張（Territorial Claims）

国境線は人為的な直線ではなく、**「山脈の尾根、大河川、大峡谷、海峡」という天然の防御障壁（自然国境）**に向けて動的に主張（Claim）が延伸する。

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                        領土主張の4つの強度区分                              │
├─────────────────────────────────────────────────────────────────────────────┤
│  [1. 近傍領土主張 (Contiguous Claim)]     : 自明の権利。他国との摩擦ゼロ。  │
│  [2. 消極的領土主張 (Passive Claim)]      : 名目上の言い置き。将来の伏線。  │
│  [3. 積極的領土主張 (Assertive Claim)]    : 外交交渉カード。バーター取引可。│
│  [4. 超積極的領土主張 (Intransigent Claim): 死活的要衝。開戦事由に直結。    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.1 領土主張の4段階マトリクス

| 区分 | 宣言コスト | 外交協議の可否 | 実効支配に必要なアクション | ゲーム上の摩擦と効果 |
| :--- | :---: | :---: | :--- | :--- |
| **近傍領土主張** | ゼロ | **不要**(自明の権利) | 調査軍団の通過、監視杭の設置 | 摩擦発生せず。領域内の失踪者集落は内政問題として平和統合可能。 |
| **消極的領土主張** | 100 CR | **可能**(容易に撤回可) | 探査部隊による地勢調査完了 | 平時は摩擦小。相手国が内乱・敗戦した際に即座に進駐する名分となる。 |
| **積極的領土主張** | 300 CR | **可能**(妥協・取引可) | 前哨基地（Outpost）建設、道路接続 | 相手国の主張と重複すると緊張度上昇。国境画定条約の外交カード。 |
| **超積極的領土主張** | 800 CR | **不可**(妥協拒否) | 軍団の恒常駐留、要塞陣地化 | **開戦事由（Casus Belli）直結**。他国進入で即座に最後通牒発令。 |

### 3.2 開戦事由（Casus Belli）の生成数式

2国間の領土主張が同一の自然国境区画で衝突した場合の緊張度摩擦値 $F_{\text{border}}$：

$$F_{\text{border}} = \sum_{\text{tiles}} \left( W_{\text{claim}}(A) \times W_{\text{claim}}(B) \times \frac{1}{\text{DistanceToCapital}} \right)$$

- $W_{\text{claim}}$: 主張強度係数（近傍: 1.0, 消極: 1.5, 積極: 3.0, 超積極: 6.0）
- $F_{\text{border}} \ge 100.0$ を突破した際、両国に**「正当な開戦事由（自衛・固有領土回復）」**が自動付与され、宣戦布告ペナルティなしで軍事制圧が可能となる。

---

## 4. Bevy ECS 実装データ構造仕様 (Rust)

```rust
use bevy::prelude::*;

/// タイルに対する領土主張コンポーネント
#[derive(Component, Debug, Clone, Reflect)]
pub struct TerritorialClaim {
    pub claiming_faction: Entity,
    pub claim_level: ClaimLevel,
    pub effective_control_score: f32, // 0.00 〜 1.00 (インフラ・駐留による既成事実度)
    pub natural_border_feature: NaturalBorderType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ClaimLevel {
    Contiguous,   // 近傍
    Passive,      // 消極的
    Assertive,    // 積極的
    Intransigent, // 超積極的（死活的）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum NaturalBorderType {
    MountainRidge, // 山脈尾根
    MajorRiver,    // 大河川
    GreatChasm,    // 大峡谷
    Coastline,     // 海岸線
    OpenPlains,    // 平原（人為境界・摩擦大）
}

/// 第三極・自発的失踪者コミュニティコンポーネント
#[derive(Component, Debug, Clone, Reflect)]
pub struct DissidentCommunity {
    pub population: u32,
    pub primary_origin_faction: Option<Entity>,
    pub hostility: f32,          // 0.00(友好的) 〜 1.00(敵対的)
    pub security_clearance: u8,  // 軍事機密隔離レベル (常時 0: 機密遮断)
    pub has_black_market: bool,
}

/// 領土主張の衝突・摩擦判定システム
pub fn evaluate_border_friction_system(
    mut commands: Commands,
    query: Query<(Entity, &TerritorialClaim)>,
    // 距離・首都位置の参照
) {
    // 1. 同一自然区画における主張重複を検出
    // 2. 超積極的主張への侵犯を検知した場合、CasusBelliEvent を発火
}
```

---

## 5. ドキュメント相互参照

- [`../planning/MISSING_SPECS_EXECUTION_PLAN.md`](../planning/MISSING_SPECS_EXECUTION_PLAN.md) - 不足ドキュメント実行計画
- [`../discussion/diplomacy_governance/concept_frontier_dissidents_and_territorial_claims.md`](../discussion/diplomacy_governance/concept_frontier_dissidents_and_territorial_claims.md) - 失踪者・領土要求検討書
- [`VICTORY_AND_CRISIS_CONDITIONS.md`](./VICTORY_AND_CRISIS_CONDITIONS.md) - 動的情勢推移・終局仕様書
- [`STEERING_UI_FLOW.md`](./STEERING_UI_FLOW.md) - 多層ステアリングUI操作フロー仕様書
