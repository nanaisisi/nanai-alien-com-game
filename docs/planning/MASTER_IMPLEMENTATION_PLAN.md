# システム統合・全体実装計画書 (Master Implementation Plan)

本ドキュメントは、これまでに確定された各種仕様書（設計思想・TO&E・兵器カタログ・資源体系・戦闘解決・技術ウェブ・生態系・UI）に基づき、本作（Nanai Alien Com Game）を段階的かつ安全にRust/Bevy 0.20 ECSアーキテクチャ上で組み上げるための**全体実装計画・詳細ロードマップ**である。

---

## 1. 現状実装状況とアーキテクチャの照合 (As-Is vs To-Be)

### 1.1 現状の実装進捗 (As-Is)

- **エンジン・ステート層 (Layer 0)**:
  - Bevy 0.20 ECS、`AppState`（Title / InGame / Settings）、コマンドライン引数による直行起動オプション（`--state` / `-s`）。
- **空間・地理基盤層 (Layer 1)**:
  - 円筒状ラップアラウンド対応ヘックスマップ（`MapGrid`, `HexCoord`）、パーリンノイズ地形生成、河川・湖沼・海洋・塩分濃度判定（`Salinity` / `WaterBodyType`）。
- **経済・インフラ層 (Layer 2)**:
  - 勢力基本データ（`FactionManager`, `PlayerFaction`）、都市メッシュ・領土システム（`TerritoryMap`, `TileTerritory`）。
  - タイル産出（`TerrainType::base_output()`）、領土内タイル在庫および勢力集計（`TileStockpile`, `FactionStockpile`, `StockpilePlugin`）。
- **軍事・戦闘団層 (Layer 3)**:
  - ユニット配置（`spawn_initial_units`）、基本移動・選択（`UnitInteractionPlugin`）。
  - 兵站データ構造（`CombatGroupLogistics`, `LogisticsState`, `MaintenanceDoctrine`）および非表示格納（`DeploymentStatus`, `HangarBay`, `NationalArsenalPool`）の基本定義。

### 1.2 目標アーキテクチャ (To-Be)

[SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md](../tech/SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md) に規定された6層構造のうち、未実装となっている「インフラ動脈」「資源加工・消費循環」「TO&E部隊運用」「戦術戦闘解決」「自律研究ウェブ」「エイリアン生態系」「ステアリングUI」を段階的に接続する。

---

## 2. 段階的実装ロードマップ (Phased Roadmap)

```text
┌────────────────────────────────────────────────────────────────────────┐
│ Phase 1: 資源流通・インフラ動脈・産業循環 (Layer 2 確立)               │
│   - タイル維持費消費と稼働率ペナルティ                                │
│   - 動脈インフラ (Transit Artery: 道路・線路・電力網) ネットワーク     │
│   - 工場タイル加工レシピ (二次中間財・兵站消費財生産)                  │
├────────────────────────────────────────────────────────────────────────┤
│ Phase 2: 戦闘団TO&E運用と非ユニット輸送 (Layer 3 確立)                 │
│   - 戦闘団 (Combat Group: 諸兵科連合コレクション `Vec<SubUnit>`)      │
│   - 兵站充足度 $S$ と状態遷移 (Normal/Strained/Critical/Exhausted)     │
│   - 海上輸送船腹プール (SLC) と陸上部隊牽引 (モーターライズ)           │
├────────────────────────────────────────────────────────────────────────┤
│ Phase 3: 自動戦闘解決と動態生態系シミュレーション (Layer 4 確立)       │
│   - 大戦略一括戦闘解決 & サブタイル戦術戦闘 (盤外砲撃・損耗・膠着)     │
│   - エイリアン巣窟 (Nest) 4段階ライフサイクル & 瘴気 (Miasma) 拡散     │
│   - 外交緊張度ラダー & 偶発事象イベントトリガー                        │
├────────────────────────────────────────────────────────────────────────┤
│ Phase 4: 技術ウェブ・国家ステアリング・情報提示 (Layer 5 確立)         │
│   - 放射状Web技術ツリー (自律研究モメンタム & 重点特許ステアリング)   │
│   - 国家ドクトリン・ステアリングUI (RFPカード・ゴーストゲージ)         │
│   - ゲーム内Wiki & リアルタイム戦況・兵站ビューア                      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. 各フェーズの詳細実装計画

### 【Phase 1】資源流通・インフラ動脈・産業循環 (Layer 2)

#### 1.1 目的

領土タイルの産出（一次資源）を加工して二次中間財・兵站消費財を生み出し、インフラ結合度を通じてタイルの維持および部隊への供給基盤を構築する。

#### 1.2 主な実装項目・データ構造

1. **タイル維持費と稼働率 (Maintenance & Efficiency)**:
   - `TileUpkeep { cost: ResourceAmounts }`
   - `TileOperatingState { efficiency: f32, is_strained: bool }`
   - ターン開始時に各タイルの備蓄から維持費を差し引き、不足時に稼働率を低下させるシステムの実装。
2. **動脈インフラ・ネットワーク (Transit Artery System)**:
   - `InfrastructureGrid`: タイル間の道路（Road）、鉄道（Rail）、送電線（PowerLine）の接続エッジ。
   - BFS/ダイクストラ法による「首都・ハブ拠点からのインフラ結合度（Connectivity: 0.0〜1.0）」算出。
   - インフラ結合度に基づくタイル間輸送能力（Transport Capacity）の制限。
3. **工場タイルと加工レシピ (Industrial Processing)**:
   - `ManufacturingPlant { capacity: f32, active_recipe: Option<RecipeId> }`
   - 一次資源（構造鉱物・エネルギー）から二次中間財（基礎部品・装甲板）や兵站消費財（C規格弾薬・燃料）への加工循環システム。

#### 1.3 検証・受け入れ基準

- 資源不足時にタイルの稼働率が低下し、産出が減少すること。
- 道路・鉄道の敷設により、後方タイルと前哨基地間のインフラ結合度が正しく算出されること。
- 工場タイルで指定レシピに従い資源が消費され、弾薬・部品が生成されること。

---

### 【Phase 2】戦闘団TO&E運用と非ユニット輸送 (Layer 3)

#### 2.1 目的

大戦略1コマの軍事ユニット（増強師団・戦闘団）を、具体的な部隊編成（`SubUnit` コレクション）およびインフラ連動兵站ステートマシンとして実体化する。

#### 2.2 主な実装項目・データ構造

1. **戦闘団TO&E構造 (Combat Group TO&E)**:
   - `SubUnit { chassis: ChassisId, module: ModuleId, condition: f32 }`
   - `CombatGroupTOE { sub_units: Vec<SubUnit>, archetype: CombatGroupArchetype }`
   - 重機甲・装輪機動・空中機動などの編成テンプレートに基づく初期編成と実数管理。
2. **兵站充足度と状態遷移 (Logistics & Maintenance)**:
   - `CombatGroupLogistics` に基づく兵站充足度 $S$ の毎ターン自動計算。
   - `LogisticsState`（Normal / Strained / Critical / Exhausted）の遷移。
   - 補給途絶時の自律共食い整備（Cannibalization）と部隊稼働率への影響。
3. **非ユニット輸送体系 (Non-Unit Transport & Sea Lift)**:
   - `FactionSeaLiftPool { total_slc: f32, allocated_slc: f32 }`
   - 港湾タイル間の海上移動時にユニットコマを出さず、SLCを消費して移動ラグ後に到着させる航行システム。
   - 徒歩・軽装歩兵戦闘団へのトラック牽引（Motorized）アタッチによる移動速度向上。

#### 2.3 検証・受け入れ基準

- 戦闘団がインフラから孤立した際、兵站充足度 $S$ が低下し、Strained/Critical へと状態遷移すること。
- 港湾間で陸上部隊を指示した場合、SLCプールを消費して海上回廊を非ユニット移動すること。
- 単体テストにて共食い整備ロジックによる部品消耗と稼働率維持の挙動が検証されていること。

---

### 【Phase 3】自動戦闘解決と動態生態系シミュレーション (Layer 4)

#### 3.1 目的

部隊同士の交戦、盤外長距離支援砲撃、エイリアン巣窟のライフサイクル、瘴気の拡散、外交緊張度の動的推移を決定論的モデルで組み上げる。

#### 3.2 主な実装項目・データ構造

1. **戦闘解決システム (Combat Resolution)**:
   - [COMBAT_RESOLUTION_AND_CASUALTY.md](../specs/COMBAT_RESOLUTION_AND_CASUALTY.md) に準拠した火力貫徹・装甲減衰モデル。
   - サブターン分割による膠着（Grinding Combat）と戦線維持判定。
   - 盤外砲撃（Off-map Artillery: 160C/210C自走榴弾砲）の着弾CEPとインフラ副次的被害。
2. **エイリアン生態・巣窟ライフサイクル (Alien Ecology & Nest)**:
   - `AlienNest { stage: NestStage, hate_counter: f32, biomass_pool: f32 }`
   - 4段階ライフサイクル（Dormant / Active / Breeding / Swarming）の自律推移。
   - 瘴気（Miasma）のグリッド自律拡散・流体モデルおよび部隊腐食ペナルティ。
3. **外交エスカレーション (Diplomatic Escalation)**:
   - `DiplomaticTension { score: f32, ladder_level: EscalationLadder }`
   - 国境未定地での摩擦、密輸、前哨基地接近による緊張蓄積と4段階ラダー判定。

#### 3.3 検証・受け入れ基準

- 戦闘発生時に部隊損耗、弾薬消費、装甲貫徹が仕様通り計算されること。
- エイリアン巣窟への刺激（熱・領土拡張）によりヘイトが蓄積し、繁殖・スウォーム行動に移行すること。
- 瘴気タイルへの進入時に通常部隊が腐食ダメージを受けること。

---

### 【Phase 4】技術ウェブ・国家ステアリング・情報提示 (Layer 5)

#### 4.1 目的

プレイヤーが直接の細かな命令ではなく「国家ドクトリン」として方向性を誘導（ステアリング）し、ゲーム世界の動態を視覚化・操作できる体験層を完成させる。

#### 4.2 主な実装項目・データ構造

1. **放射状Web技術ツリー (Tech Web System)**:
   - [TECHNOLOGY_WEB_DATA_TREE.md](../specs/TECHNOLOGY_WEB_DATA_TREE.md) に準拠した全ノード依存グラフ。
   - 現場刺激による「自律研究モメンタム」と指導者の「重点特許ステアリング」。
   - 3大アフィニティ（Purity / Harmony / Supremacy）の累積・解禁判定。
2. **国家ドクトリン・ステアリングUI (Steering Dashboard)**:
   - RFP 3軸スライダー（防護/機動/火力）および共通化拘束UI。
   - 工廠ライン転換時の金型移行ラグ（ゴーストゲージ）表示。
   - 現場ボトムアップ改修（Field Mod）承認ポップアップ。
3. **ゲーム内Wiki & ライブビューア (In-Game Wiki & Live Monitor)**:
   - 部隊TO&E、兵器カタログ、資源辞書のインゲーム閲覧ビューア。
   - リアルタイム戦況・兵站供給ライン・損耗統計の表示。

#### 4.3 検証・受け入れ基準

- 技術ノードの研究完了により、対応モジュールや施設が解禁されること。
- ステアリングUIのスライダー変更に伴い、国内工廠の設計RFPが時間をかけて移行すること。
- ゲーム内Wikiから各種兵器・資源・ルールの仕様が参照できること。

---

## 4. 開発プロセスと品質保証規律

1. **テスト駆動・回帰防止**:
   - ロジック変更時は単体テスト（Unit Tests）を必ず同伴させ、`RUSTUP_TOOLCHAIN=1.97.1 cargo test` の全件パスを維持する。
2. **ドキュメント整合性の維持**:
   - 実装に伴い仕様に微調整が生じた場合は、即座に該当の `docs/specs/` および `docs/INDEX.md` を更新する。
3. **Markdown規約の遵守**:
   - 表内での `<br>` タグ使用禁止（MD033）、相対パス規約、Prettierフォーマットを徹底する。
