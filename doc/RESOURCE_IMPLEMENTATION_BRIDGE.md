# 資源系 実装橋渡し案 (Resource Implementation Bridge)

設計（[RESOURCE_ECONOMY_SYSTEM.md](./RESOURCE_ECONOMY_SYSTEM.md)）と現行コードの間を埋め、段階的に実装するための暫定案です。確定仕様ではなく、叩き台です。

※リンクはすべてリポジトリ内相対パスです。

---

## 1. 現状 (As-Is)

| 項目 | 実装状況 |
| :--- | :--- |
| タイル産出 | `TerrainType::base_yields()` が `(食料, 生産, エネルギー, 科学)` を `f32` で返す（`src/map/terrain.rs`） |
| 表示 | 各タイル上に四捨五入した整数で表示。`MapDisplaySettings.show_yields` 初期ON、`Y` キーで切替（`src/map/interaction.rs`, `src/map/types.rs`） |
| 集計・消費 | 未実装（表示のみ） |
| 資源の型 | `RawResource` / `ManufacturedGood` / `MilitaryConsumable` は設計のみ。コード上は未定義 |

## 2. 目標 (To-Be)

一次資源 → 二次中間財 → 兵站消費財 → 部隊・タイル維持、という連鎖（詳細は設計書の第1章）。

## 3. 橋渡し：プレイヤー向け4産出と内部資源の対応

**方針：わかりやすさ優先。** プレイヤーが見る基本産出は現行の4種（食料・生産・エネルギー・科学）のまま変えず、内部の資源品目は裏側で対応づけます。科学と希少鉱物は別物として扱います。

| タイル産出（表示） | 内部資源 | 備考 |
| :--- | :--- | :--- |
| 食料 | `Biomass` | 1:1 |
| エネルギー | `Energy` | 1:1 |
| 生産 | `StructuralOre` | 構造鉱物の採掘・加工力を「生産」として見せる |
| 科学 | 研究ポイント（資源品目ではない） | 技術研究専用。備蓄・加工の対象にしない |
| （表示なし） | `RareMinerals` | 基本4産出には含めない。**特殊資源**として特定タイルにのみ存在（例：結晶地帯） |

- 基本4産出は全タイルの共通指標、希少鉱物は「持っているタイルだけアイコンが出る」特殊資源にして、数字の種類を増やしすぎない。
- 希少鉱物は二次中間財（論理コア・複合装甲）の原料。Phase 4 以降で導入する。

## 4. データモデル案

内部値は `f32`、表示のみ整数丸め（現行方針を維持）。

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RawResource { Energy, StructuralOre, RareMinerals, Biomass }

/// 基本産出。タイル産出・収支で共通利用（科学は研究ポイントとして別枠）
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceAmounts { pub energy: f32, pub ore: f32, pub biomass: f32, pub science: f32 }

/// 特殊資源（希少鉱物など）。該当タイルのみ Some
pub struct TileSpecialResource { pub kind: RawResource, pub amount: f32 }


/// 戻り値を f32 タプルから置換する想定
impl TerrainType { pub fn base_output(&self) -> ResourceAmounts { /* ... */ } }

#[derive(Resource, Default)]
pub struct FactionStockpile(pub HashMap<FactionId, ResourceAmounts>);
```

- `base_yields()` は移行期間中、`base_output()` から変換する薄いラッパとして残す。
- 表示側は `ResourceAmounts` を丸めて文字列化するだけにする。

## 5. 確定方針（2026-10-17）

- **備蓄単位：タイル/拠点ごとの在庫**を持ち、輸送で移動させる。勢力単位の国庫は「各タイル在庫の合計表示」として派生させる（実体は持たない）。
- **「生産」産出＝構造鉱物の採掘量のみ。** 二次中間財の加工は別枠で、**工場タイルの加工能力値**（ターンあたり処理量）で行う。

### 5.1 資源の偏在・供給依存（現実の寡占を軽く再現する方針）

現代の資源偏在や中間財の寡占（特定国への依存）は、**新システムを足さず、既存設計の組み合わせだけで表現する**。細かい国別依存度の計算は行わない。

| 要素 | 実現方法 |
| :--- | :--- |
| 偏在 | `RareMinerals` は特殊資源タイルにのみ存在（追加ルールなし） |
| 寡占 | 論理コア等の一部中間財は、対応する工場タイルを持つ勢力しか加工できない。他勢力は輸送・交易で入手 |
| 途絶 | 依存元が敵対・封鎖・被災すると、輸送（Phase 4）・補給途絶（Phase 6）のルールで自然に稼働率が下がる |

- 国別の依存度の可視化（輸入比率・依存先表示、制裁連動）は**後回し**。必要になった時点で追加する。

## 6. 段階的実装計画

| Phase | 内容 | 完了条件 |
| :--- | :--- | :--- |
| 0 | タイル産出の整数表示（完了） | 表示ON、内部f32 |
| 1 | `RawResource` / `ResourceAmounts` 導入、`base_output()` へ置換 | 表示が現行と同一 |
| 2 | 領土タイルごとの在庫 `TileStockpile` へ毎ターン産出を加算。勢力合計をUI表示（領土は `src/faction/territory.rs`） | UIに合計を表示 |
| 3 | 維持費消費（タイル進展・部隊の稼働率）と赤字時のペナルティ | 資源不足で稼働率低下 |
| 4 | 輸送（タイル間の在庫移動、インフラ結合度による容量制約） | 在庫が移動する |
| 5 | 二次中間財の加工（工場タイルの加工能力値） | 変換レシピが動作 |
| 6 | 兵站消費財（弾薬・燃料）と補給途絶 | 前哨の補給途絶が発生 |

## 7. 未決事項

- 希少鉱物タイルの表示方法（専用アイコン／産出テキスト追記／ホバー表示のみ。未回答）
- タイル在庫の保管上限と、輸送の容量・所要ターン数
- 工場タイルの加工能力値の決め方（タイル進展Tier依存か）
- 産出の小数端数の扱い（累積は `f32` のまま、表示のみ丸め／ターン境界で丸めるか）
- 資源タイル（結晶地帯等）の個体差：地形固定値か、生成時のランダム係数か
- 保管上限・損耗（腐敗する食料など）の導入時期

## 8. 関連ドキュメント

- [RESOURCE_ECONOMY_SYSTEM.md](./RESOURCE_ECONOMY_SYSTEM.md)
- [concept_resource_economy_and_logistics.md](./discussion/industry_logistics/concept_resource_economy_and_logistics.md)
- [concept_tile_progression_system.md](./discussion/map_tile/concept_tile_progression_system.md)
- [SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md](./SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md)
