# 資源・部品・弾薬マスター定義書 (Data Dictionary)

## (Resource, Materials & Munitions Physical Data Dictionary)

本ドキュメントは、本作（4X大戦略＋諸兵科連合戦闘団制＋脱都市タイル進展）における**「一次資源、二次中間財、兵站消費財」の物理的妥当性（質量・容積・工学根拠）と、ゲーム計算・ECSデータ構造としての軽量性（集約バルク管理）を両立させたデータ辞書仕様書**である。

---

## 1. 物理的妥当性と軽量設計の基本原則

### 1.1 質量基準（トン：$t$）による物理的裏付け

ゲーム内の抽象ポイントを排除し、すべて**「重量（t：トン）」および「電力量（MWh）」**を共通基準として統一する。
これにより、「戦車1両（50〜60t）の生産に必要な鉄鉱石量」「1個師団（1万人・車両600両）が1日に消費する食料（20〜30t）や砲弾（数百t）」という現実の軍事・工学的なスケール感が直感的に整合する。

### 1.2 計算・操作負荷（コスト）を重くしない集約ルール

1. **マイクロトラッキングの排除**:
   - 車両1台ごとの砲弾数・燃料リッター数を追跡しない。
   - 戦闘団（師団）全体の**「弾薬残量（t）」「燃料残量（t）」「交換部品（t）」の3大バルクプール**として一括管理する。
2. **整数・単純乗算によるECS更新**:
   - 消費計算は $\text{消費量} = \text{基礎消費率} \times \text{部隊規模係数} \times \text{行動強度係数}$ の整数・単純乗算で毎ターン1回のみバッチ処理。
3. **プレイヤー認知の直感化**:
   - UI上では重量数値とともに「**作戦持続可能ターン数（例: 残り 4 ターン）**」を併記し、プレイヤーが電卓を叩く必要を一切排除する。

---

## 2. 一次資源（Primary Resources）マスター定義

土地の物理特性から産出される基礎原料。

| 資源名 (英語) | 物理単位 | 標準タイル産出レンジ (通常 Tier 1〜2) | 高度開発 (Tier 3〜4) / エイリアン濃縮鉱床 | 備蓄方式 | 物理的妥当性・世界観背景 |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **エネルギー**(Energy) | `MWh` | `50 〜 150 MWh / T` | `300 〜 800 MWh / T` | 蓄電施設 / 送電網 | 地熱・太陽光・核融合。長距離送電で毎タイル3%の送電ロスが発生。 |
| **構造鉱物**(Structural Ore) | `t` (トン) | `100 〜 300 t / T` | `500 〜 2,500 t / T` (巣窟跡地は富鉱帯) | タイル集積場 / 鉱山 | 鉄、チタン、ボーキサイト等の構造材用粗鉱石。山岳・丘陵およびエイリアン濃縮巣窟に集約。※埋蔵量制限なし（恒久産出）。 |
| **機能性鉱物**(Rare Minerals) | `t` (トン) | `20 〜 60 t / T` | `100 〜 600 t / T` (生体晶析嚢胞) | 密閉サイロ / 工廠 | レアアース、半導体母材、リチウム。エイリアンが甲殻・感覚器用に生体濃縮。※埋蔵量制限なし。 |
| **バイオ原質**(Biomass) | `t` (トン) | `50 〜 200 t / T` | `200 〜 1,500 t / T` | 低温貯蔵サイロ | 森林伐採、水域藻類、現地植物・生体組織。食料および合成燃料の炭素源。 |

---

## 3. 二次中間財（Intermediate Goods）加工マスター定義

工廠やコンビナートで加工される工業部材。加工ロスを工学的に反映（歩留まり 85〜95% 想定）。

| 中間財名 (英語) | 単位 | 標準加工レシピ (1単位生産) | 投入工数 | 物理的妥当性・用途 |
| :--- | :---: | :--- | :---: | :--- |
| **基礎機械部品**(Standard Parts) | `t` | 構造鉱物 1.2t ＋ 0.5 MWh | 1 ターン | 切削ロス（200kg）を経て製造されるボルト、ギア、油圧配管、車軸フレーム。兵器・インフラの基礎。 |
| **電子・論理コア**(Logic Cores) | `kg` | 機能鉱物 0.2t ＋ 1.5 MWh | 2 ターン | 耐電磁波FCS、センサ素子、自律AIプロセッサ。質量は軽微（箱単位）だが電力・希少資源の集約体。 |
| **複合装甲板**(Composite Armor) | `t` | 構造鉱物 0.8t ＋ 機能鉱物 0.3t ＋ 1.0 MWh | 1 ターン | セラミック・チタン合金・ケブラー混成装甲。戦車（40〜50t装甲）や陣地防壁に大量消費。 |
| **規格化レーション**(Standard Rations) | `t` | バイオ原質 1.1t ＋ 0.2 MWh | 1 ターン | 水分調整・滅菌された長期保存食料。1t ＝ 約500人日分の熱量・栄養素。 |

---

## 4. 兵站消費財（Logistics Consumables）消費モデル

諸兵科連合戦闘団（増強師団：兵員10,000名、戦闘車両600両規模）を基準とする1ターンあたりの質量消費・運用モデル。

### 4.1 C規格弾薬 (Munitions)

- **物理単位**: `t` (トン)
- **製造レシピ**: 基礎機械部品 0.7t ＋ 機能鉱物 0.1t ＋ エネルギー 0.4 MWh ➔ **弾薬 1.0t**
- **師団標準搭載量**: **1,500 t**（最大継戦能力: 約3〜5激戦ターン分）

#### 【工学的・質量試算メモ：小火器弾薬と師団消費の妥当性根拠】

- **単弾重量とトン換算**:
  - 歩兵標準の小口径実包（`6C` ≒ 6×50C弾）を薬莢・装薬込みで **約 12.5 g / 発** と仮定。
  - $1.0\,\text{t} = 1,000,000\,\text{g} \div 12.5\,\text{g} =$ **約 80,000 発 / t**。
- **歩兵1名の標準携行数**:
  - 30発マガジン × 4本 ＋ 銃本体装着1本 ＝ **計 150 発（約 1.88 kg）**。
- **1戦闘・1ターンあたりの消費規模（師団10,000名）**:
  - 1回の小競り合い・局地接敵において、「撃ち切らない程度に撃つ前衛」と「撃たない後方」を均して **平均 50 発 / 名** と概算。
  - 師団全体で一度に交戦する前線歩兵が約3,000〜4,000名の場合：
    $3,500\,\text{名} \times 50\,\text{発} = 175,000\,\text{発} \approx 2.2\,\text{t}$（純小火器弾薬）。
  - **1ターン（数日〜数週間の作戦期間）への拡張**:
    大戦略の1ターン内では局地戦闘が数倍〜数十倍繰り返され、さらに車載重機（`13C`）・機関砲（`30C`）・迫撃砲（`80C/120C`）の斉射重量が急激に跳ね上がる。
    そのため、師団全体の1ターン消費が「小競り合いで数十t」「通常接敵で150〜250t」「特科・重砲全力斉射で400〜600t」となる設定は、物理質量として極めて妥当なスケール感となる。

- **消費モデル（1ターン・1個師団あたり）**:
  - **待機・平時駐留**: ほぼ `0 t`
  - **前進・威力偵察（小火器・機銃中心）**: `30 〜 50 t`
  - **通常接敵戦（30C機関砲・80C滑腔砲交戦）**: `150 〜 250 t`
  - **全力突破・師団特科砲撃戦（160C/210C特科斉射・ミサイル連射）**: `400 〜 600 t`

### 4.2 推進燃料 (Fuel / Propellant)

- **物理単位**: `t` (トン)
- **製造レシピ**: バイオ原質 0.8t ＋ エネルギー 0.6 MWh ➔ **合成炭化水素燃料 1.0t**
- **師団標準搭載量**: **2,000 t**（車両600両＋直協ヘリ32機）
- **消費モデル（1ターン・1個師団あたり）**:
  - **陣地待機・哨戒アイドリング**: `50 t`
  - **舗装路・幹線道路行軍（インフラ結合度 0.8 以上）**: `150 t`
  - **未踏地・不整地走破（泥濘・山岳・インフラ結合度 0.3 未満）**: `350 〜 450 t`
  - **強襲機動・渡河突破作戦**: `500 t`

### 4.3 野戦交換部品 (Field Spares)

- **物理単位**: `t` (トン)
- **製造レシピ**: 基礎機械部品 0.8t ＋ 電子コア 50kg ＋ 0.3 MWh ➔ **交換部品 1.0t**
- **師団標準搭載量**: **500 t**（履帯ブロック、転輪、油圧シリンダ、予備砲身、交換センサー）
- **消費モデル（損耗回復レート）**:
  - **通常行軍摩耗**: `10 〜 20 t / T`
  - **泥濘・酸性瘴気走破**: `40 〜 60 t / T`
  - **戦闘損耗の現地急速再生（軽度損害大隊の復旧）**: `80 〜 150 t / 回`

### 4.4 規格化レーション（食料）と異星適応度

- **師団標準消費**: **25 t / T**（1日2.5kg × 10,000名 × 1ターン）
- **異星度（Alien Contamination / Adaptation %）による影響**:
  - **母星純粋食料（異星度 0%）**: 補給コスト最大、医療費ゼロ、兵士士気高。
  - **現地バイオ適応食料（異星度 50%）**: 原生植物由来。輸送コスト半減、軽微な消化器疾患により交換部品・医療品消費 +10%。
  - **瘴気菌類・極限現地食料（異星度 90%以上）**: 現地自給100%可能。ただし長期運用で「適応種分化（変異）」が進行。

---

## 5. Bevy ECS 実装データ構造仕様 (Rust)

軽量化のため、全資源・消費財は浮動小数点ではなく固定整数（キログラム換算または整数トン）として扱い、計算オーバーヘッドを最小化する。

```rust
use bevy::prelude::*;

/// 質量ベースの一次資源プール（国家または拠点タイル）
#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct TileResourceStockpile {
    /// 蓄電量 (MWh)
    pub energy_mwh: u32,
    /// 構造鉱物 (トン: t)
    pub structural_ore_tons: u32,
    /// 機能性鉱物 (トン: t)
    pub rare_minerals_tons: u32,
    /// バイオ原質 (トン: t)
    pub biomass_tons: u32,
}

/// 中間財ストック（工廠・兵站基地）
#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct IndustrialWarehouse {
    pub standard_parts_tons: u32,
    pub logic_cores_count: u32, // 個数/箱単位
    pub composite_armor_tons: u32,
    pub rations_tons: u32,
}

/// 戦闘団（師団）が保有する兵站3大バルクプール（集約管理）
#[derive(Component, Debug, Clone, Reflect)]
pub struct CombatGroupSupplyPool {
    /// 弾薬ストック (t) / 最大搭載容量 (t)
    pub ammo_current_tons: u16,
    pub ammo_max_capacity_tons: u16,

    /// 燃料ストック (t) / 最大搭載容量 (t)
    pub fuel_current_tons: u16,
    pub fuel_max_capacity_tons: u16,

    /// 交換部品ストック (t) / 最大搭載容量 (t)
    pub spares_current_tons: u16,
    pub spares_max_capacity_tons: u16,

    /// レーション食料ストック (t)
    pub rations_current_tons: u16,
    pub rations_max_capacity_tons: u16,

    /// 異星食料比率 (0 〜 100%)
    pub alien_diet_ratio: u8,
}

impl CombatGroupSupplyPool {
    /// 標準師団（定員規模）の満載時初期設定
    pub fn new_standard_division() -> Self {
        Self {
            ammo_current_tons: 1500,
            ammo_max_capacity_tons: 1500,
            fuel_current_tons: 2000,
            fuel_max_capacity_tons: 2000,
            spares_current_tons: 500,
            spares_max_capacity_tons: 500,
            rations_current_tons: 150, // 約6ターン分
            rations_max_capacity_tons: 150,
            alien_diet_ratio: 0,
        }
    }

    /// 行軍時の燃料消費判定（高速・単純除算）
    pub fn consume_fuel_for_transit(&mut self, distance_tiles: u32, infra_connectivity: f32) {
        let base_burn = 150 * distance_tiles;
        let penalty = if infra_connectivity >= 0.8 {
            1.0
        } else if infra_connectivity >= 0.4 {
            1.8
        } else {
            2.5 // 未踏地・泥濘
        };
        let consumed = (base_burn as f32 * penalty) as u16;
        self.fuel_current_tons = self.fuel_current_tons.saturating_sub(consumed);
    }

    /// 戦闘ラウンドでの弾薬消費判定
    pub fn consume_ammo_in_combat(&mut self, combat_intensity: CombatIntensity) {
        let burn_tons = match combat_intensity {
            CombatIntensity::Skirmish => 40,
            CombatIntensity::StandardEngage => 180,
            CombatIntensity::FullBarrageBreakthrough => 500,
        };
        self.ammo_current_tons = self.ammo_current_tons.saturating_sub(burn_tons);
    }

    /// 残り持続可能ターン数の算出（UI表示用）
    pub fn estimate_sustainable_combat_turns(&self) -> u16 {
        // 標準戦闘 (180t/T) を想定
        self.ammo_current_tons / 180
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatIntensity {
    Skirmish,                  // 軽微な小競り合い
    StandardEngage,            // 通常接敵戦闘
    FullBarrageBreakthrough,   // 全力突破・特科斉射
}
```

---

## 6. ドキュメント相互参照

- [`../planning/MISSING_SPECS_EXECUTION_PLAN.md`](../planning/MISSING_SPECS_EXECUTION_PLAN.md) - 不足ドキュメント実行計画
- [`RESOURCE_ECONOMY_SYSTEM.md`](./RESOURCE_ECONOMY_SYSTEM.md) - 資源・経済・サプライチェーン体系目録
- [`STEERING_UI_FLOW.md`](./STEERING_UI_FLOW.md) - 多層ステアリングUI操作フロー仕様書
- [`../units/force/COMBAT_GROUP_LOGISTICS_INTEGRATION.md`](../units/force/COMBAT_GROUP_LOGISTICS_INTEGRATION.md) - 戦闘団兵站・共食い整備
