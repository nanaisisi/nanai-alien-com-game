# SEC-08: 戦闘解決・部隊損耗および弾薬消費計算仕様書

本仕様書は、諸兵科連合戦闘団（TO&E）同士、および野生エイリアン群との交戦において、自動戦闘解決（Auto-Resolve）を行う際の数理モデルと部隊の損耗状態遷移、およびECS（Entity Component System）データ構造について規定する。

---

## 1. 自動戦闘解決数理モデル（Combat Resolution Math Model）

大戦略上の親タイル上で戦闘が発生した場合、または親タイル内のサブターン（戦術フェーズ）を自動計算で進行する場合の基本的なダメージ解決式を定義する。

### 1.1 基本火力と命中判定

戦闘の最小単位（小隊または個体単位）での射撃・攻撃計算。

$$ \text{Hit Chance} = \text{Base Accuracy} \times (1 - \text{Cover Modifier}) \times (1 - \text{Stealth / Weather Modifier}) $$

- **Base Accuracy（基本命中率）**: 兵装の種別（機関砲、戦車砲、ミサイル、格闘）とFCS（火器管制）世代に基づく基本値。
- **Cover Modifier（地形遮蔽減衰）**: サブタイルの地形（森林、山岳、都市廃墟）による命中低下ペナルティ。
- **Stealth / Weather Modifier（天候・光学迷彩減衰）**: 光学迷彩（Gen 3+ ステルス）や悪天候、瘴気による命中率低下。

### 1.2 ダメージ貫徹・減衰計算

攻撃が命中した場合のダメージ計算は、「火力貫徹力」と「装甲防護値」の対比で行う。

$$ \text{Damage} = \text{Base Damage} \times \max\left(0.1, \frac{\text{Firepower}}{\text{Armor Value} + \text{Hardness Modifier}}\right) $$

- **Firepower（火力貫徹力）**: 運動エネルギー弾（APFSDS）、化学エネルギー弾（HEAT）、あるいはエイリアンの生体酸・巨大質量による貫徹力指数。
- **Armor Value（装甲防護値）**: 車両の装甲厚（RHA換算）、複合装甲、生体甲殻の防御力。
- **Hardness Modifier**: 車両の部位（正面、側面、上面）や、歩兵の防弾装備による補正。エイリアンの格闘攻撃は装甲値に対して固定のボーナスを持つ（組み付きによる装甲破壊を表現）。

---

## 2. 損耗（Casualty）と部隊状態遷移

戦闘団を構成するサブユニット（歩兵分隊、車両小隊）の損耗判定。

### 2.1 下車歩兵の損耗

歩兵のHP（または人数）が0になった場合、以下の判定を行う。

- **Dead（戦死） / KIA**: 即座に戦力外。補充には長期間と訓練が必要。
- **Wounded（負傷） / WIA**: 戦力外だが、野戦病院（衛生ユニット）での治療により早期復帰（または共食い補充）が可能。対酸・生体毒の被害はWIAになりにくい。

### 2.2 戦闘車両の損耗状態

車両へのダメージ蓄積により、以下のステータスへ移行する。

1.  **Fully Operational（完全稼働）**: 100%〜80%の耐久度。ペナルティなし。
2.  **Lightly Damaged（軽度損傷）**: 79%〜40%の耐久度。移動力・命中率に小ペナルティ。
3.  **Immobilized / Disabled（擱座・行動不能）**: 39%〜1%の耐久度。移動不能。装甲・火器は一部機能する可能性があるが、戦線維持には寄与しない。ARV（装甲回収車）による回収・修理対象。
4.  **Destroyed（全損）**: 0%。完全破壊。修復不可だが、スクラップ・資源として回収可能。

### 2.3 弾薬・燃料の消費

交戦ごとに、兵装の消費レートに応じて実際の弾薬（C規格弾薬、ミサイル）と推進剤（燃料）の在庫を減少させる。

- **弾薬枯渇時（Bingo Ammo）**: 火力（Firepower）が10%に低下（近接火器・自衛火器のみ）。
- **燃料枯渇時（Bingo Fuel）**: 移動力（Mobility）が0になり、回避率低下。

---

## 3. 膠着（Grinding Combat）と戦線維持

大規模な戦闘（軍団規模）は1ターンで決着しないことが多い。

### 3.1 決着と膠着判定

1親ターン（内部的に12〜15サブターン）の終了時、以下の条件で判定を行う。

- **突破 / 全滅**: 一方の戦力（またはモラル）が崩壊した場合、勝者がタイルを占有。
- **膠着（Grinding Combat）**: 双方の戦力が一定以上残存し、前線が固定化された状態。次親ターンへ戦闘が持ち越される。

### 3.2 後退・撤退判定と増援ラグ

- **戦線崩壊**: 特定の部隊（中隊・大隊）の損耗率が50%を超えると、モラルチェックが発生し、失敗すると隣接タイルへ後退（Retreat）する。
- **増援進入ラグ**: 隣接タイルにいる味方部隊が戦場サブタイルに合流するまでには、インフラ網（道路・鉄道）と移動力に応じた遅延ターン（サブターン）が発生する。

---

## 4. 盤外長距離支援砲撃（Off-map Artillery）

別タイルに配置された特科・ミサイル部隊からの支援攻撃ロジック。

### 4.1 着弾散布界（CEP）とダメージ

指定したサブタイルを中心とする着弾散布界（CEP: Circular Error Probable）を計算する。

- **誘導弾（Missile/Guided）**: CEPが小さく、特定ユニットへのピンポイントダメージ。
- **無誘導榴弾（Artillery）**: CEPが広く、中心から距離に応じてダメージが減衰（スプラッシュダメージ）。

### 4.2 インフラ・タイルへの付随的被害（Collateral Damage）

砲撃は敵ユニットだけでなく、タイルのインフラ（道路、居住区、送電網）にもダメージを与える。

- 重砲（160C/210C）による支援砲撃を乱用すると、占領後のタイルTierが後退（デグレード）し、復旧に多大なコストがかかる。

---

## 5. ECS データ構造（Rust/Bevy）実装案

```rust
// 戦闘ユニットのベース状態
#[derive(Component)]
pub struct UnitCombatStats {
    pub base_accuracy: f32,
    pub firepower: f32,
    pub armor_value: f32,
    pub hp: f32,
    pub status: VehicleStatus,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VehicleStatus {
    Operational,
    LightlyDamaged,
    Immobilized,
    Destroyed,
}

// 弾薬・兵站状態
#[derive(Component)]
pub struct UnitLogistics {
    pub ammo_c_standard: u32,
    pub ammo_missile: u32,
    pub fuel: u32,
}

// 親タイルの戦闘状態
#[derive(Component)]
pub struct TileCombatState {
    pub is_grinding: bool,
    pub sub_turn_count: u8,
    pub attacker_morale: f32,
    pub defender_morale: f32,
}
```
