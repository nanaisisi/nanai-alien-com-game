# SEC-09: 技術ウェブ・研究開発データツリー仕様書

## (Technology Web, Radial Research & Affinity Progression Specification)

本ドキュメントは、本作（4X大戦略＋諸兵科連合戦闘団制＋脱都市タイル進展）における研究開発（R&D）システムである**「放射状技術ウェブ（Radial Tech Web）」**の完全データ構造、36の全技術ノード定義、3大アフィニティ（適応思想）および3大ハイブリッド進化マイルストーン、現場自律研究モメンタムと国家ステアリングの定量的数理モデル、ならびに戦場残骸解析（Forensics）と勢力別初期研究特性を規定する基本仕様書である。

---

## 1. 放射状技術ウェブ構造（Radial Tech Web）

従来の4Xゲームにみられる単一線形ツリー（一本道）を排し、中央の「降着基盤（Core / Tier 0）」を中心として同心円状・全方位に広がる放射状（Radial Web）構造をとる。

```text
                       【調和・共生 (Harmony)】
                     （異星生体融合・瘴気環境適応）
                                  ▲
                                  │
                               [Tier 3]
                                  │
                               [Tier 2]
                                  │
  【超越・自律 (Supremacy)】 ── [Tier 1] ── 【純血・母星化 (Purity)】
  （完全無人・量子知性・AI）     │     (Landing Core)    （重化学工業・テラフォーミング）
                             [Tier 0]
                                  │
                               [Tier 1]
                                  │
                               [Tier 2]
                                  │
                               [Tier 3]
                                  ▼
                       【極限物理・深層開発】
```

### 1.1 階層構造（Trunk & Leaf）

- **幹技術 (Trunk)**:
  - 包括的な学問的ブレイクスルーおよび産業パラダイム転換。
  - 所要研究ポイント（RP）が高く、新時代の前提条件となる。各Tierの中核ノードを形成する。
- **葉技術 (Leaf)**:
  - Trunkから派生する具体的兵器モジュール、車台、生産レシピ、特殊ドクトリン。
  - 所要RPは比較的低く、即効性のある戦力強化・産業歩留まり向上をもたらす。

### 1.2 4大技術ティア（Tier 0〜3）と兵器世代（Gen 0〜4）対応

| 技術Tier   | 時代区分名                     | 兵器世代対応  | 世界観・産業フェーズ                                            | 基礎必要RPレンジ |
| :--------- | :----------------------------- | :------------ | :-------------------------------------------------------------- | :--------------- |
| **Tier 0** | **降着基盤 (Colony Core)**     | Gen 0 / Gen 1 | 母星宇宙船解体物資の活用、初期開拓、生存と基礎加工の立ち上げ    | 0 〜 60 RP       |
| **Tier 1** | **基礎拡張・産業規格化**       | Gen 2         | 独立した現地重工業の確立、C規格弾薬・共通車台の量産開始         | 100 〜 200 RP    |
| **Tier 2** | **アフィニティ特化・構造転換** | Gen 3         | 3大思想への分化、複合装甲・高度電子戦・生体触媒リアクターの稼働 | 250 〜 500 RP    |
| **Tier 3** | **極限物理・惑星特異点**       | Gen 4         | 惑星環境の完全改変、巨大陸上戦艦、完全無人軍団、タイタン使役    | 600 〜 1400 RP   |

---

## 2. 全36技術ノード完全カタログ

ゲーム内で実装される全36ノードの完全データ定義。全国家共通の出発点（Tier 0）から、Purity（純血）、Harmony（調和）、Supremacy（超越）の3軸、およびそれらの交差領域（ハイブリッド）へと分岐する。

### 2.1 Tier 0: 降着基盤 (Colony Core / 5ノード)

初期開拓と生存のための共通基盤技術群。

| ノードID    | 種別  | 技術名 (和名 / 英名)                      | 前提ノード  | 必要RP | 解禁要素 (兵器モジュール / 施設 / レシピ / TO&E)                   | アフィニティ付与 |
| :---------- | :---- | :---------------------------------------- | :---------- | :----- | :----------------------------------------------------------------- | :--------------- |
| `TECH_CORE` | Trunk | 降着基盤工学 (Colony Landing Core)        | なし        | 0      | 探査バギー、基礎インフラ、6C/8C小火器、Std-W6/T4車台、民兵軍団TO&E | なし             |
| `TECH_C_01` | Leaf  | 船体解体冶金 (Salvage Metallurgy)         | `TECH_CORE` | 50     | 構造鉱物回収、粗製溶接装甲 (ARM-01)、基礎機械部品レシピ            | Purity +1        |
| `TECH_C_02` | Leaf  | 密閉水耕循環 (Hydroponic Recirc)          | `TECH_CORE` | 50     | 規格化レーションレシピ、初期温室ドーム、食料自給施設               | なし             |
| `TECH_C_03` | Leaf  | 地表波レーダー網 (Planetary Radar Survey) | `TECH_CORE` | 50     | 資源探査視界、瘴気・断層タイルの可視化、WPN-03 (同軸8C)            | Supremacy +1     |
| `TECH_C_04` | Leaf  | 原生生体サンプリング (Biosphere Sampling) | `TECH_CORE` | 50     | バイオ原質採取リグ、携帯型解毒剤パック、生体分析ドック             | Harmony +1       |

### 2.2 Tier 1: 基礎拡張・産業規格化 (Frontier Standardization / 9ノード)

現地重工の立ち上げと産業規格化（C規格体系、共通車台）の確立。

| ノードID      | 種別  | 技術名 (和名 / 英名)                          | 前提ノード    | 必要RP | 解禁要素 (兵器モジュール / 施設 / レシピ / TO&E)                 | アフィニティ付与 |
| :------------ | :---- | :-------------------------------------------- | :------------ | :----- | :--------------------------------------------------------------- | :--------------- |
| `TECH_TRK_P1` | Trunk | 高張力重工冶金 (Heavy Machinery Engineering)  | `TECH_C_01`   | 180    | Gen 1/2 履帯車台、粗鋼重工高炉、装甲化軍団TO&E、HETトレーラー    | Purity +2        |
| `TECH_LF_P11` | Leaf  | 共通ボルト・規格化令 (Standardized Fasteners) | `TECH_TRK_P1` | 90     | 部品共通化率+15%、工廠段取り替え工期 -1ターン、交換部品消費-10%  | Purity +1        |
| `TECH_LF_P12` | Leaf  | C規格火器・迫撃砲架 (Caliber-C Munitions)     | `TECH_TRK_P1` | 110    | WPN-02 (80C低圧破砕砲)、120C自走迫撃砲、C規格弾薬専用加工ライン  | Purity +2        |
| `TECH_TRK_H1` | Trunk | 原生生物ゲノム解析 (Alien Biome Sequencing)   | `TECH_C_04`   | 180    | 瘴気観測所、キチン外骨格切削施設、バイオ採取リグTier 2           | Harmony +2       |
| `TECH_LF_H11` | Leaf  | 瘴気解離エアロゾル (Miasma Dissociation)      | `TECH_TRK_H1` | 90     | 除染キット、軽歩兵の瘴気非戦闘損耗-50%、酸中和弾                 | Harmony +1       |
| `TECH_LF_H12` | Leaf  | 現地生体飼料精製 (Bio-Feedstock Extraction)   | `TECH_TRK_H1` | 100    | 異星動植物からのレーション変換（食料産出+25%）、合成燃料代替精製 | Harmony +2       |
| `TECH_TRK_S1` | Trunk | 分散計算知性 (Distributed Cybernetics)        | `TECH_C_03`   | 180    | 電子・論理コア加工ライン、基礎戦術データリンク、NC工作機械       | Supremacy +2     |
| `TECH_LF_S11` | Leaf  | 自律NC工作機械群 (Automated NC Tools)         | `TECH_TRK_S1` | 90     | 町工場の生産スループット+20%、工作機械プール拡張                 | Supremacy +1     |
| `TECH_LF_S12` | Leaf  | 無人監視センサ網 (Unmanned Surveillance Net)  | `TECH_TRK_S1` | 100    | WPN-04 (4連多目的ミサイル)、監視杭、無人小型偵察ドローン小隊     | Supremacy +2     |

### 2.3 Tier 2: アフィニティ特化・構造転換 (Paradigm Divergence / 13ノード)

3大アフィニティの本格的分岐と、2大アフィニティが交差するハイブリッド技術の解禁。

| ノードID       | 種別  | 技術名 (和名 / 英名)                           | 前提ノード                   | 必要RP | 解禁要素 (兵器モジュール / 施設 / レシピ / TO&E)                   | アフィニティ付与   |
| :------------- | :---- | :--------------------------------------------- | :--------------------------- | :----- | :----------------------------------------------------------------- | :----------------- |
| `TECH_TRK_P2`  | Trunk | 結晶合金・重装甲技術 (Crystalline Metallurgy)  | `TECH_TRK_P1`                | 460    | Gen 3 主力戦車車台、複合装甲板量産 (ARM-02)、160C重自走榴弾砲      | Purity +4          |
| `TECH_LF_P21`  | Leaf  | 大気改変スクラバー (Atmospheric Scrubber)      | `TECH_TRK_P2`                | 220    | 固定式大気浄化塔、隣接タイルの瘴気永久消滅、母星種子農場           | Purity +3          |
| `TECH_LF_P22`  | Leaf  | 重軌道動脈鉄道 (Heavy Strategic Rail)          | `TECH_TRK_P2`                | 230    | TRN-02 (重軌道装甲列車)、鉄道幹線インフラ、戦略再配置コスト半減    | Purity +3          |
| `TECH_TRK_H2`  | Trunk | 生体触媒・神経共鳴 (Biocatalytic Synapse)      | `TECH_TRK_H1`                | 460    | 有機触媒リアクター、常温バイオ精錬、多種族混成軍団TO&E解禁         | Harmony +4         |
| `TECH_LF_H21`  | Leaf  | 擬態フェロモン弾 (Pheromone Ordnance)          | `TECH_TRK_H2`                | 220    | 誘引フェロモン弾（野生エイリアン誘導）、フェロモン散布キャニスター | Harmony +3         |
| `TECH_LF_H22`  | Leaf  | 異星藻類光合成槽 (Alien Algae Photobioreactor) | `TECH_TRK_H2`                | 230    | 高密度バイオ燃料プラント、酸素濃縮施設、食料自給力大幅向上         | Harmony +3         |
| `TECH_TRK_S2`  | Trunk | 光論理演算・統合射撃網 (Optical C4I Networks)  | `TECH_TRK_S1`                | 460    | 超微細半導体ファブ、全域統合データリンク、光速FCS、AESAレーダー    | Supremacy +4       |
| `TECH_LF_S21`  | Leaf  | 自律二脚歩行駆動系 (Autonomous Bipedal Walker) | `TECH_TRK_S2`                | 240    | Std-L4車台、二脚歩行戦闘車両モジュール、高級装甲・二脚歩行混成軍団 | Supremacy +3       |
| `TECH_LF_S22`  | Leaf  | 高出力マイクロ波兵装 (HPM Disruption Grid)     | `TECH_TRK_S2`                | 230    | HPM指向性エネルギー兵装、対ドローンスウォーム迎撃網、電子撹乱装置  | Supremacy +3       |
| `TECH_HYB_PS1` | Leaf  | 電熱化学砲ETC (Electro-Thermal Chemical Gun)   | `TECH_TRK_P2`, `TECH_TRK_S1` | 260    | 120C/140C ETC戦車砲、貫徹力+40%、Mecha-Purity初等ドクトリン        | Purity +2, Sup +2  |
| `TECH_HYB_PH1` | Leaf  | キチン重工複合装甲 (Chitin-Steel Composite)    | `TECH_TRK_P2`, `TECH_TRK_H1` | 250    | キチン鋼複合装甲モジュール（耐酸・耐熱+50%）、Bio-Foundry初等      | Purity +2, Harm +2 |
| `TECH_HYB_HS1` | Leaf  | サイバネ神経直結 (Neural Cyber-Link)           | `TECH_TRK_H2`, `TECH_TRK_S1` | 260    | エイリアン遠隔サイバネ統御、神経リンクアビオニクス、Bio-Cyber初等  | Harm +2, Sup +2    |
| `TECH_HYB_PS2` | Leaf  | 自動装填砲塔システム (Autoloading Turret FCS)  | `TECH_TRK_P1`, `TECH_TRK_S2` | 250    | 完全自動装填160C砲塔、乗員削減(-25%)、連射速度+35%                 | Purity +2, Sup +2  |

### 2.4 Tier 3: 極限物理・惑星特異点 (Apex Singularities / 9ノード)

終局勝利条件および国家の命運を決する超先端技術・決戦兵器群。

| ノードID      | 種別  | 技術名 (和名 / 英名)                             | 前提ノード                    | 必要RP | 解禁要素 (兵器モジュール / 施設 / レシピ / TO&E)                     | アフィニティ付与 |
| :------------ | :---- | :----------------------------------------------- | :---------------------------- | :----- | :------------------------------------------------------------------- | :--------------- |
| `TECH_TRK_P3` | Trunk | 惑星テラフォーミング (Planetary Remodeling)      | `TECH_TRK_P2`                 | 1300   | 惑星改造中枢プラント、テラフォーミング勝利プロジェクト、軌道アンカー | Purity +6        |
| `TECH_LF_P31` | Leaf  | 陸上戦艦・超大口径砲 (Land Battleship & Railgun) | `TECH_TRK_P3`, `TECH_HYB_PS1` | 650    | 陸上戦艦打撃軍団TO&E、EM-210C超大型電磁投射砲、2タイル超砲撃         | Purity +4        |
| `TECH_LF_P32` | Leaf  | 成層圏オゾン再形成網 (Stratospheric Restoration) | `TECH_TRK_P3`                 | 600    | 紫外線・通信障害恒久消滅、母星型大都市区画整備、人口幸福度向上       | Purity +4        |
| `TECH_TRK_H3` | Trunk | 惑星ガイア同調 (Ecological Awakening)            | `TECH_TRK_H2`                 | 1300   | 惑星意識同調勝利プロジェクト、生体共振中枢核、瘴気無害化             | Harmony +6       |
| `TECH_LF_H31` | Leaf  | 生体タイタン共鳴神経 (Titan Neural Harness)      | `TECH_TRK_H3`, `TECH_HYB_HS1` | 650    | 超大型タイタン級（コロッサス）直接使役、融合生物戦闘団TO&E           | Harmony +4       |
| `TECH_LF_H32` | Leaf  | 自己再生生体外皮 (Regenerative Bio-Armor)        | `TECH_TRK_H3`                 | 600    | 戦闘中・非戦闘時自然急速再生（毎ターン修復+25%）、酸噴射砲           | Harmony +4       |
| `TECH_TRK_S3` | Trunk | 完全自律オムニ知性 (Omni-Autonomous Intellect)   | `TECH_TRK_S2`                 | 1300   | 全土電脳統御勝利プロジェクト、AI司令部、完全自律戦闘団TO&E           | Supremacy +6     |
| `TECH_LF_S31` | Leaf  | 指向性高出力レーザー (High-Energy Laser Grid)    | `TECH_TRK_S3`, `TECH_LF_S22`  | 650    | HEL要塞迎撃システム、ミサイル・航空機即時撃墜、弾薬消費ゼロ兵装      | Supremacy +4     |
| `TECH_LF_S32` | Leaf  | 分散量子知性ファブ (Quantum Autonomous Fabs)     | `TECH_TRK_S3`                 | 600    | 無人完全自動生産（国内全工廠スループット+40%）、工作機械無制限       | Supremacy +4     |

---

## 3. アフィニティ（Affinity）進化マイルストーン体系

各アフィニティ（Purity, Harmony, Supremacy）の獲得ポイントが一定値に達するごとに、国家のアフィニティレベル（Lv 1〜10）が昇格し、恒久的なドクトリン恩恵が解禁される。

### 3.1 アフィニティレベル昇格条件

累計アフィニティポイントに基づく必要閾値テーブル：

| アフィニティLv | 必要累積ポイント | 解禁マイルストーン概要                                           |
| :------------: | :--------------: | :--------------------------------------------------------------- |
|    **Lv 1**    |     **2 pt**     | 思想の芽生え：基礎ドクトリンバフ（下表参照）                     |
|    **Lv 2**    |     **5 pt**     | 初期特化：専任研究機関の設立・現場モメンタム加速+15%             |
|    **Lv 3**    |     **9 pt**     | 第1次兵器近代化：第1世代固有モジュールアンロック                 |
|    **Lv 4**    |    **14 pt**     | 産業再編：特化中間財の生産歩留まり+10%                           |
|    **Lv 5**    |    **20 pt**     | 中期ドクトリン転換：共通車台の専用派生型解禁                     |
|    **Lv 6**    |    **27 pt**     | 第2次兵器近代化：師団TO&Eの固有発展型編成アンロック              |
|    **Lv 7**    |    **35 pt**     | 特化インフラ：国家規模の専任メガストラクチャー解禁               |
|    **Lv 8**    |    **44 pt**     | 決戦兵器配備：Tier 3 決戦兵装・超大型アセット解禁                |
|    **Lv 9**    |    **54 pt**     | イデオロギー極大化：国家民心統一、全都市出力+15%                 |
|   **Lv 10**    |    **65 pt**     | **惑星特異点突破：固有勝利条件（エンディング）プロジェクト解禁** |

### 3.2 3大アフィニティ別 マイルストーン効果一覧

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                           3大アフィニティの進化対比                         │
├─────────────────────────────────────────────────────────────────────────────┤
│  [Purity (純血)]    : 重機甲・超重砲・テラフォーミング・大気浄化            │
│  [Harmony (調和)]   : 生体装甲・酸毒兵装・瘴気回復・多種族協調・タイタン    │
│  [Supremacy (超越)] : 無人自律機・データリンク・レーザー電磁砲・量子知性    │
└─────────────────────────────────────────────────────────────────────────────┘
```

| 系統          | Lv 3 特権                                                                | Lv 6 特権                                                                                | Lv 10 特権 (終局勝利プロジェクト)                                                                      |
| :------------ | :----------------------------------------------------------------------- | :--------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| **Purity**    | **均質鋼溶接強化**: 全車両の装甲値+15%、交換部品生産コスト-15%           | **重機甲突撃ドクトリン**: 重戦車・重自走榴弾砲の弾薬消費効率+25%、舗装路移動力+1         | **「母星復権テラフォーミング」**: 惑星大気を完全浄化し旧地球環境を再建、母星移民船団を受け入れ主権勝利 |
| **Harmony**   | **瘴気順応呼吸**: 師団の瘴気移動ペナルティ消滅、瘴気内滞在で部隊自然回復 | **多種族共生ドクトリン**: エイリアン種族の師団直接編制解禁、キチン装甲の重量-20%         | **「ガイア意識統合」**: 惑星ハイブマインド中枢と全人類の意識を共振・融合させ精神的超克勝利             |
| **Supremacy** | **分散射撃統御**: 隣接部隊からの支援射撃命中率+20%、索敵範囲+1           | **完全無人前衛ドクトリン**: 前衛部隊の乗員損失ペナルティ完全ゼロ化、無人機スウォーム運用 | **「全土電脳統御」**: 惑星全域のインフラ・防衛網を単一の量子自律AI中枢に統合し機械的支配勝利           |

### 3.3 3大デュアル・ハイブリッドアフィニティ

2つのアフィニティを並行して高める（双方Lv 4以上達成）ことで解禁される混成ドクトリン。

1. **Mecha-Purity（Purity × Supremacy: 高密度自動重火工）**:
   - 母星型の重火砲・重装甲プラットフォームに完全自律射撃管制（FCS）と無人自動装填を統合。
   - 効果: 160C/210C重火砲の射撃準備ターン不要化、砲兵の反撃被弾率-50%。
2. **Bio-Foundry（Purity × Harmony: 強化母星生態系）**:
   - 母星の動植物・作物遺伝子に異星生体の極限環境耐性・生体触媒をハイブリッド交配。
   - 効果: 微小瘴気タイルの農場化（食料自給力+40%）、キチン鋼複合装甲の生産コスト-25%。
3. **Bio-Cyber（Harmony × Supremacy: 生体サイバネティクス）**:
   - 生体神経組織と超微細シリコン論理回路の直結。
   - 効果: 野生エイリアンのサイバネ遠隔使役、航空バイオアビオニクスによるドッグファイト回避率+30%。

---

## 4. 自律研究モメンタムと国家ステアリングの定量的数理モデル

本作の研究開発は、プレイヤーによる万能な神視点選択ではなく、**「現場の自律的な研究進展（ボトムアップ）」**と**「指導者の政策誘導（トップダウン）」**の合成ベクトルとして進行する。

### 4.1 現場起因の自律研究モメンタム (Research Momentum)

毎ターン、各技術ノードへ自律的に注入される基礎研究進捗ポイント $\Delta \text{RP}_{\text{auto}}(i)$ は、以下の数式で決定論的に算出される：

$$\Delta \text{RP}_{\text{auto}}(i) = \text{CombatStimulus}(i) + \text{IndustrialStimulus}(i) + \text{EnvironmentalStimulus}(i)$$

1. **交戦刺激 (Combat Stimulus)**:
   - 実戦での弾薬消費量および被弾状況に応じて関連ノードに加算。
   - 実弾・重砲使用ターン: 火砲・冶金ノードに $+1.0 \times \frac{\text{弾薬消費量(t)}}{100}$
   - エイリアン交戦・酸被弾ターン: 生体解析・キチンノードに $+2.0 \times \text{交戦部隊数}$
   - 無人ドローン・ミサイル使用ターン: 自律制御・通信ノードに $+1.5 \times \text{使用回数}$
2. **産業構造刺激 (Industrial Stimulus)**:
   - 国内保有タイルの地目（Zoning）および稼働施設に連動。
   - 高炉・鉱山コンビナート: 重工冶金系ノードに $+1.0 \times \text{施設Tier総和}$
   - バイオリアクター・温室: 生体ゲノム・生体触媒ノードに $+1.0 \times \text{施設Tier総和}$
   - 精密ファブ・発電所: 分散計算・光論理ノードに $+1.0 \times \text{施設Tier総和}$
3. **環境刺激 (Environmental Stimulus)**:
   - 領土内に存在する過酷タイルの比率に連動。
   - 瘴気被覆タイル保有: 瘴気解離・生態系ノードに $+0.5 \times \text{瘴気タイル数}$
   - 断層・地震帯タイル保有: 耐震・地殻工学ノードに $+1.0 \times \text{活断層タイル数}$

### 4.2 国家指導者のステアリング介入 (Steering Controls)

プレイヤーは国家予算および科学アカデミーの重点特許指定により、特定ノードに国家研究力 $\text{RP}_{\text{national}}$ を集中配分する。

$$\Delta \text{RP}_{\text{steer}}(i) = \begin{cases} \text{RP}_{\text{national}} \times (1.0 - \text{FrictionPenalty}), & \text{if } i = \text{TargetNode} \\ 0, & \text{otherwise} \end{cases}$$

#### 研究転換フリクション (R&D Friction)

現在進んでいる産業・アフィニティ傾向と全く異なる研究を重点指定した場合、研究者コミュニティの再編のため**移行停滞ターン（Friction Turns）**が発生する。

$$\text{FrictionTurns} = \max\left(0, \left\lfloor \frac{|\text{CurrentAffinity} - \text{TargetAffinity}|}{2} \right\rfloor \right)$$

- 移行停滞期間中、ステアリングによる注入RPは**50%に減衰**する（$\text{FrictionPenalty} = 0.50$）。
- 移行ターンが経過すると、研究陣の再編が完了し100%の効率でRPが注入される。

---

## 5. 戦場残骸法医学（Forensics）とリバースエンジニアリング

戦闘終了後、激戦地タイルに残された敵兵器・撃破エイリアンの残骸から得られる技術ブーストの算定式。

### 5.1 残骸解析ボーナス算定式

工兵または調査戦闘団が残骸タイルで「戦場精査（Forensics）」を実行した際、獲得する即時RP：

$$\text{BonusRP}(i) = \text{BaseYield} \times (1.0 + \Delta \text{Tier} \times 0.50) \times \text{QualityFactor}$$

- **BaseYield**: 軽車両残骸＝20 RP、主力戦車残骸＝60 RP、超大型・タイタン残骸＝150 RP。
- **$\Delta \text{Tier}$**: 自国の現行保有Tierと敵残骸のTier差（$\Delta \text{Tier} \ge 1$ の格上兵器ほど高ボーナス）。
- **QualityFactor**: 自爆（Scuttling）成功時は 0.30、完全撃破時は 0.70、機能停止拿捕時は 1.00。

### 5.2 サプライチェーン制約（リバースエンジニアリングの限界）

知見（RP）を獲得して技術ノードがアンロックされても、**自国の工作機械プール（Machine Tool Pool）や中間財サプライチェーンが未整備である場合、工廠での量産は不可能**である。
上位兵器の量産には、対応する重工冶金（Layer 2）および高精度ファブ（Layer 1）の稼働が前提となる。

---

## 6. 架空6大国家の初期研究特性・アフィニティ指向

[`specs/Faction.md`](./Faction.md) および [`assets/data/factions.csv`](../../assets/data/factions.csv) で定義された6大国の初期保有技術と研究指向マトリクス。

| 勢力コード | 勢力名                    | 初期解禁技術             | 主力研究指向 (モメンタム補正)           | 初期得意アフィニティ                            |
| :--------: | :------------------------ | :----------------------- | :-------------------------------------- | :---------------------------------------------- |
|   **A**    | **帝国** (Empire)         | `TECH_CORE`, `TECH_C_01` | 重火砲・高張力冶金・大口径実弾 (+30%)   | **Purity** (母星至上・重装甲)                   |
|   **B**    | **大公国** (Grand Duchy)  | `TECH_CORE`, `TECH_C_03` | センサ・光論理・極地耐寒工学 (+30%)     | **Supremacy / Purity** (技術・プラグマティズム) |
|   **C**    | **連邦** (Federation)     | `TECH_CORE`, `TECH_C_02` | 農業循環・共通車台・大量生産 (+30%)     | **Purity / Harmony** (大地・多様性生産)         |
|   **D**    | **共和国** (Republic)     | `TECH_CORE`, `TECH_C_01` | 粗鋼重工・鉄道インフラ・計画動員 (+30%) | **Purity** (集権動員・重工業)                   |
|   **E**    | **共同体** (Commonwealth) | `TECH_CORE`, `TECH_C_04` | 異星生体採取・海洋養殖・生体触媒 (+30%) | **Harmony** (環境適応・バイオ産業)              |
|   **F**    | **連合** (Union)          | `TECH_CORE`, `TECH_C_03` | 自律知性・NC自動化・航空電子 (+30%)     | **Supremacy** (完全自動化・量子AI)              |

---

## 7. Bevy ECS 実装データアーキテクチャ (Rust)

ゲームエンジン（Bevy 0.20+）上で本システムを稼働させるためのRustデータ構造設計。

```rust
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// 3大アフィニティ蓄積ポイント
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub struct AffinityScore {
    pub purity: u32,
    pub harmony: u32,
    pub supremacy: u32,
}

impl AffinityScore {
    pub fn level_of(&self, affinity_type: AffinityType) -> u8 {
        let pts = match affinity_type {
            AffinityType::Purity => self.purity,
            AffinityType::Harmony => self.harmony,
            AffinityType::Supremacy => self.supremacy,
        };
        match pts {
            0..=1 => 0,
            2..=4 => 1,
            5..=8 => 2,
            9..=13 => 3,
            14..=19 => 4,
            20..=26 => 5,
            27..=34 => 6,
            35..=43 => 7,
            44..=53 => 8,
            54..=64 => 9,
            _ => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AffinityType {
    Purity,
    Harmony,
    Supremacy,
}

/// 技術ノード定義
#[derive(Debug, Clone, Reflect)]
pub struct TechNodeDefinition {
    pub id: String,
    pub name_ja: String,
    pub name_en: String,
    pub tier: u8,
    pub is_trunk: bool,
    pub prerequisites: Vec<String>,
    pub required_rp: u32,
    pub affinity_gain: AffinityScore,
    pub unlocks: Vec<String>,
}

/// 全技術ノードレジストリ
#[derive(Resource, Debug, Clone, Default)]
pub struct TechWebRegistry {
    pub nodes: HashMap<String, TechNodeDefinition>,
}

/// 国家の研究開発コンポーネント
#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct FactionResearchProgress {
    pub unlocked_techs: HashSet<String>,
    pub accumulated_rp: HashMap<String, u32>,
    pub research_momentum: HashMap<String, f32>,
    pub steering_target: Option<String>,
    pub steering_friction_turns: u8,
}
```

---

## 8. ドキュメント相互参照

- [GAME_DESIGN.md](../GAME_DESIGN.md) - ゲーム基本設計書
- [specs/INDUSTRIAL_AND_TECH_TREE_SYSTEM.md](./INDUSTRIAL_AND_TECH_TREE_SYSTEM.md) - 産業システムおよび技術ツリー試案仕様書
- [specs/RESOURCE_ECONOMY_SYSTEM.md](./RESOURCE_ECONOMY_SYSTEM.md) - 資源・経済・サプライチェーン体系目録
- [specs/CHASSIS_AND_MODULE_CATALOG.md](./CHASSIS_AND_MODULE_CATALOG.md) - 初期モジュール・基本車台実数値カタログ
- [specs/Faction.md](./Faction.md) - 登場勢力設定仕様書
- [discussion/world_progression/concept_technology_tree_structure.md](../discussion/world_progression/concept_technology_tree_structure.md) - 研究開発・技術ウェブ設計仕様書
- [discussion/world_progression/concept_three_pillar_tech_progression.md](../discussion/world_progression/concept_three_pillar_tech_progression.md) - 3大並列技術体系仕様書
