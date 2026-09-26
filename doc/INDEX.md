# ドキュメント構成・目録 (Documentation Index)

本プロジェクト（Nanai Alien Com Game）における各種仕様書・検討資料の構成および索引です。

---

## 📁 全体ディレクトリ構造

```text
doc/
├── INDEX.md                                # 本ファイル（ドキュメント目録）
├── GAME_DESIGN.md                          # ゲーム企画・コア仕様設計書
├── Faction.md                              # 勢力一覧・固有ボーナス/特性定義
├── PENDING_ISSUES.md                       # 未確定・検討課題まとめ
├── MISSING_DOCUMENTS_ROADMAP.md            # 未作成・不足ドキュメント体系と開発ロードマップ
└── discussion/                             # 詳細設計・発展仕様・アイデア検討ドキュメント群（計40件）
    ├── diplomacy_governance/               # 外交・占領統治・戦後処理（6件）
    │   ├── concept_diplomacy_relations.md
    │   ├── concept_alliance_system.md
    │   ├── concept_frontier_dissidents_and_territorial_claims.md
    │   ├── concept_occupation_governance_and_divided_economy.md
    │   ├── concept_capitulation_remnants_and_supply_chain.md
    │   └── concept_factions_detail_and_naming.md
    ├── events/                             # イベント・ナラティブシステム（5件）
    │   ├── concept_event_situational.md
    │   ├── concept_event_stochastic.md
    │   ├── concept_event_catalog_and_chains.md
    │   ├── concept_faction_consensus_and_standardization.md
    │   └── concept_diplomatic_incidents_and_crises.md
    ├── industry_logistics/                 # 産業・サプライチェーン・組織（10件）
    │   ├── concept_resource_economy_and_logistics.md
    │   ├── concept_industry_structure.md
    │   ├── concept_industry_dynamics_novelty_continuity.md
    │   ├── concept_industrial_evolution_and_supply_chains.md
    │   ├── concept_bootloader_and_national_supply_chains.md
    │   ├── concept_industry_module_mapping.md
    │   ├── concept_doctrine_and_equipment_steering.md
    │   ├── concept_automation_systems.md
    │   ├── concept_transport_windows_and_infrastructure_bottlenecks.md
    │   └── concept_theater_operations_and_joint_sectors.md
    ├── map_tile/                           # マップ・サブタイル・領土拡張（6件）
    │   ├── sub_tile_seamless_connection.md
    │   ├── future_concept_subtile_industry.md
    │   ├── concept_tile_centric_expansion.md
    │   ├── concept_tile_progression_system.md
    │   ├── concept_maritime_governance_and_ocean_engineering.md
    │   └── concept_subtile_combat_and_tactical_rules.md
    ├── military_tactics/                   # 軍事・兵器モジュール・戦術運用（7件）
    │   ├── concept_weapon_module_commonality.md
    │   ├── example_weapon_modules_cases.md
    │   ├── concept_future_legged_unmanned_rules.md
    │   ├── concept_ethics_autonomous_and_surrender_dynamics.md
    │   ├── concept_salvage_scuttling_and_compatibility.md
    │   ├── concept_dynamic_arsenal_geospatial.md
    │   └── concept_combat_group_structure_and_rfp_cycle.md
    ├── ui_wiki/                            # UI・情報閲覧システム（3件）
    │   ├── concept_in_game_wiki.md
    │   ├── concept_viewer_separation_wiki_and_live.md
    │   └── concept_doctrine_steering_ui.md
    └── world_progression/                  # 世界観・生態系・進行フェーズ（6件）
        ├── concept_frontier_civilization_and_ecosystem_dynamics.md
        ├── concept_demographics_ectogenesis_and_speciation.md
        ├── concept_environmental_dynamics_and_future_energy.md
        ├── concept_tectonics_volcanism_and_earthquake.md
        ├── concept_technology_tree_structure.md
        ├── concept_game_progression_and_victory.md
        └── concept_alien_lifecycle_and_nest_mechanics.md
```

---

## 📑 1. ルート基本ドキュメント

* **[GAME_DESIGN.md](./GAME_DESIGN.md)**：ゲーム企画・基本仕様設計書（マスタードキュメント）
  * ゲームコンセプト（4X SFターン制戦略、Civ:BEライク）
  * 全体マップ（大戦略層）と単一タイルマップ（戦術層）の二重スケール構造
  * 固定編成・戦闘団制（増強師団相当）による小部隊運用
  * 原住エイリアンとの共存／支配／排除の関係性システム
  * 技術ツリー、内政・資源、外交の基本方針
* **[Faction.md](./Faction.md)**：登場勢力設定仕様書
  * 母星の歴史・大戦停戦を経て入植した架空の6大国家概念（帝国A、大公国B、連邦C、共和国D、共同体E、連合F）
  * 各国の体制・文化圏背景、母星と入植惑星での外交関係（具体的な国名は未確定・継続検討事項）
  * アフィニティ（思想・適応路線）や経済/軍事ボーナスの基本方向性
* **[PENDING_ISSUES.md](./PENDING_ISSUES.md)**：未確定・検討課題まとめ（論点整理）
  * 歩行兵器の定義と不便さの具現化、国家概念の肉付け、戦闘団×モジュール共通化の接続など、掘り下げ不足な論点と今後の検討課題一覧。
* **[MISSING_DOCUMENTS_ROADMAP.md](./MISSING_DOCUMENTS_ROADMAP.md)**：未作成・不足ドキュメント体系と開発ロードマップ
  * 全体ドキュメントの最新成熟度マトリクス、明確に不足している5大領域（資源品目定義、技術ツリー、外交危機、エイリアン生態巣窟、ステアリングUI）、および開発ロードマップ。

---

## 💡 2. 詳細検討・発展仕様（`doc/discussion/`）

各システム・メカニクスごとに最新のサブディレクトリ分類（7分野）で体系的に管理されています。

### ① マップ構造・サブタイル運用・領土拡張（`map_tile/`）

全体マップ（大戦略）とサブタイル（戦術マップ）の連携、および「脱・都市中心」のタイル拡張システム。

* **[sub_tile_seamless_connection.md](./discussion/map_tile/sub_tile_seamless_connection.md)**
  * 全体マップとサブタイル（戦術マップ）のシームレスな接続性。
  * タイル境界（エッジ）での部隊進入・離脱・追撃、隣接タイルからの増援・支援砲撃の物理的結合ルール。
* **[future_concept_subtile_industry.md](./discussion/map_tile/future_concept_subtile_industry.md)**
  * サブタイル上に存在する小規模施設・インフラ（パイプライン、送電線、観測所、防衛タワー等）の配置と、戦術戦での破壊・防衛インタラクション。
* **[concept_tile_centric_expansion.md](./discussion/map_tile/concept_tile_centric_expansion.md)**
  * **脱・都市中心主義（Post-City Centric）のタイル個別拡張システム**。
  * 万能な「都市」オブジェクトを廃止し、各タイルを「軍事前哨(FOB)」「資源採掘」「重工業コンビナート」「居住区画」等の個別機能サイトとして定義。
  * 道路・送電線・パイプラインの物理的インフラ延伸による開拓と、343サブタイルの段階的設営（設営→基礎化→特化→自律集積）、変電所や補給パイプラインのピンポイント破壊による兵站戦。
* **[concept_tile_progression_system.md](./discussion/map_tile/concept_tile_progression_system.md)**
  * **タイル共通フォーマットと進展度（Progression Tier 0〜4）システム**。
  * 全タイル共通の最小スキーマ（インフラ結合度、稼働率、実効支配度、環境負荷、サブグリッド）。
  * 4大区分（産業、資源、社会居住、軍事）× 5段階進展（未開地→仮設前哨→本格運用→高度集積→要塞中枢）の進化マッピングと、補給途絶や戦火による後退・荒廃（デグレード）の緊張感。
  * **タイル単位の占有権と部隊配置ルール**: 都市を介さない直接占有、個別部隊増殖を防ぐ「1タイル1部隊（アクティブ）」の原則、移動不能な余剰スタックの戦闘力喪失・兵站負担加速（自壊・連鎖降伏リスク）。
  * **隣接タイルの干渉と戦場過密（「みちみち」の大混戦）**: 平時は1部隊でも、戦闘時は周囲6方向の隣接タイルから敵味方の増援がエッジより殺到し、343サブタイルが過密化。射線フリクションや交通麻痺、戦闘後の再編と個別部隊の降伏・残存判定。
* **[concept_maritime_governance_and_ocean_engineering.md](./discussion/map_tile/concept_maritime_governance_and_ocean_engineering.md)**
  * **海上統治・海洋工学仕様検討書**。
  * 埋め立て、固定式、半固定式、メガフロートの4大工法比較と水深別適性。
  * 海上都市（立体レイヤード構造）の市民生活、動力途絶による沈没パニック、塩害・生体付着、水中音共鳴によるリヴァイアサン級エイリアン誘引リスク。
  * 海洋タイルのTier 0〜4進展、シーレーン防衛、公海・国境未定地におけるメガフロート曳航による領海既成事実化（グレーゾーン紛争）。
* **[concept_subtile_combat_and_tactical_rules.md](./discussion/map_tile/concept_subtile_combat_and_tactical_rules.md)**
  * **サブタイル戦術戦闘・ターン構造および境界増援仕様書**。
  * 親マップ1ターン＝最大12〜15サブターンの時間分割、未決着交戦（Grinding Combat）の膠着と次ターン持ち越し。
  * 隣接親タイルからの増援進入ラグ（地形・インフラ依存）と盤外長距離支援砲撃（Off-Map Artillery/Missiles）。
  * 外周エッジにおける戦術的離脱・追撃、敵陣突破（Breakthrough）、戦術マップ上の施設破壊による大戦略親タイルTierへの即時フィードバック。

### ② 軍事・兵器モジュール・戦術運用（`military_tactics/`）

ユニットの設計、兵器モジュール化、無人・自律兵器、戦場ルール。

* **[concept_weapon_module_commonality.md](./discussion/military_tactics/concept_weapon_module_commonality.md)**
  * 車体／シャシーと兵装モジュールの共通化設計（モジュラー兵器システム）。生産効率と改修自由度の両立。
* **[example_weapon_modules_cases.md](./discussion/military_tactics/example_weapon_modules_cases.md)**
  * 兵装モジュール・シャシー組み合わせの実装具体例（対空砲架、レールガン、ミサイルポッド、電子戦装備等）。
* **[concept_future_legged_unmanned_rules.md](./discussion/military_tactics/concept_future_legged_unmanned_rules.md)**
  * 多脚・二脚歩行兵器の特性検討（万能ではなく構造の複雑さ・整備負荷・脆弱性等の「不便さ」を伴うロマン枠としての議論深化）、無人ドローン・自律機械ユニットの階層（Tier）と通信途絶時の行動ルーチン。
* **[concept_ethics_autonomous_and_surrender_dynamics.md](./discussion/military_tactics/concept_ethics_autonomous_and_surrender_dynamics.md)**
  * 自律兵器運用における倫理的課題・国際条約、敵部隊の降伏判定と捕虜・無力化ユニットの処理。
* **[concept_salvage_scuttling_and_compatibility.md](./discussion/military_tactics/concept_salvage_scuttling_and_compatibility.md)**
  * 戦場での残骸回収（サルベージ）、鹵獲阻止のための自爆・自沈（スカットリング）、異文明・敵性技術モジュールの規格互換性とリバースエンジニアリング。
* **[concept_dynamic_arsenal_geospatial.md](./discussion/military_tactics/concept_dynamic_arsenal_geospatial.md)**
  * 地理・地形特性に応じた戦力展開、補給工廠（アーセナル）の空間的配置と動的兵站ラインの防衛。
* **[concept_combat_group_structure_and_rfp_cycle.md](./discussion/military_tactics/concept_combat_group_structure_and_rfp_cycle.md)**
  * **戦闘団スロット構造・RFP自動更新および現場改修仕様書**。
  * 戦闘団の固定16スロットTO&E（基幹打撃4、機械化歩兵4、間接火力3、偵察自律3、兵站2）。
  * 国家RFP指示を受けた国内工廠の自動ブロック更新（Block I → II）と金型転換ラグ。
  * 過酷実戦環境が生むボトムアップ現場改修（Field Mod）と国家の制式採用承認イベント。
  * 新型・旧型混在による過渡期稼働率ペナルティ算出モデルと計画ロット更新の重要性。

### ③ 産業・サプライチェーン・組織運用（`industry_logistics/`）

生産ライン、モジュール製造、煩雑さを軽減する自動化システム、および広域指揮統治。

* **[concept_resource_economy_and_logistics.md](./discussion/industry_logistics/concept_resource_economy_and_logistics.md)**
  * **資源カテゴリ・物資格納・食料と弾薬規格の進化流通仕様書**。
  * 冷戦直後型地球共通規格から、同時並行開発による分化、後発追従・同盟協定・デファクトスタンダードによる再統合に至る弾薬規格の動的メカニクス。
  * 陸（土壌・温室・畜産）／海（閉鎖養殖・海洋資源）／工業（細胞培養リアクター）の食料3大アプローチ、気候タイルの自活投資、および異星現地食品（あるものを食うリスク）。
  * 一次資源（エネルギー、構造/機能鉱物、バイオ原質）、二次中間財、軍事・兵站消費財の3層品目定義。
* **[concept_industry_structure.md](./discussion/industry_logistics/concept_industry_structure.md)**
  * 素材採掘・精錬・中間部品加工・最終製品組立に至る多段階産業ツリーの基本構造。
* **[concept_industry_dynamics_novelty_continuity.md](./discussion/industry_logistics/concept_industry_dynamics_novelty_continuity.md)**
  * **新規性と継続性からみる産業構造仕様書**。
  * 既存規格・工作機械・金型・高歩留まりによる「継続性の慣性・長寿命プラットフォーム」と、新世代AI・新物性による「新規性の特異点・初期脆弱性」の二項対立、および産業移行フェーズ。
* **[concept_industrial_evolution_and_supply_chains.md](./discussion/industry_logistics/concept_industrial_evolution_and_supply_chains.md)**
  * **文明・産業革命史からみる産業エコシステムとサプライチェーン創出仕様書**。
  * 「軍事は産業の氷山の一角（民生基盤の海に浮かぶ突起）」という脱・軍事偏重モデル。
  * 歴史的産業革命の4大エポック（粗放開拓期 → 重化学規格化期 → 電子情報精密ファブ期 → 極限物理・自律バイオ統合期）。
  * 露光装置・量子論理ファブ等の「世界で数拠点の頂点寡占（Apex Monopoly）」と、町工場・レガシー半導体等の「各地に分散する裾野（Pervasive Base）」のピラミッド構造。
  * 民生需要から立ち上がるサプライチェーンの自己組織化、デュアルユース（軍民両用）転用性、および国家産業政策。
* **[concept_bootloader_and_national_supply_chains.md](./discussion/industry_logistics/concept_bootloader_and_national_supply_chains.md)**
  * **機械を作る機械（ブートローダー産業）と国家サプライチェーン仕様書**。
  * 母機・工作機械・金型と歩留まり（Yield）の力学、他国規格・鹵獲兵器の「自国化改修（国産化）」およびライセンス生産の現実的制約。
* **[concept_doctrine_and_equipment_steering.md](./discussion/industry_logistics/concept_doctrine_and_equipment_steering.md)**
  * **国家制御・多層ステアリングシステム仕様書**。
  * マクロ大戦略・戦域・個別兵器クラス・現場改修の4層ステアリングアーキテクチャ。
  * 主力装甲・IFV・砲兵・無人機等の兵器クラス別3軸RFPスライダー（防護/機動/火力）と共通化拘束。
  * 産業の慣性、工廠金型転換の移行ラグ（Tooling Friction）、過渡期整備ペナルティ、および現場ボトムアップ改修（Block Field）の追認・制式化。
* **[concept_automation_systems.md](./discussion/automation/concept_automation_systems.md)**
  * 自動輸送ルート設定、定期哨戒（パトロール）、前線への自動補給ロジック。
* **[concept_transport_windows_and_infrastructure_bottlenecks.md](./discussion/industry_logistics/concept_transport_windows_and_infrastructure_bottlenecks.md)**
  * **輸送ウインドウ（タイムスロット）とインフラボトルネック検討仕様**。
  * 道路・鉄道・海上・航空・軌道の各輸送モードにおける時間枠・ノード/エッジ制約の構造的比較。
* **[concept_theater_operations_and_joint_sectors.md](./discussion/industry_logistics/concept_theater_operations_and_joint_sectors.md)**
  * **方面軍（Theater Command）システムと自動指揮・管理・占領統治**。
  * 専念領域（Dedicated Sector）と方面間共同管理領域（Joint Sector）の分離・連携メカニクス。

### ④ 外交・占領統治・戦後処理（`diplomacy_governance/`）

勢力間の関係性、戦争終了後の国家解体と統治メカニクス。

* **[concept_diplomacy_relations.md](./discussion/diplomacy_governance/concept_diplomacy_relations.md)**
  * 国家間外交、通商協定、防衛協定、エイリアンに対する共同方針。
* **[concept_alliance_system.md](./discussion/diplomacy_governance/concept_alliance_system.md)**
  * 3大思想ベースの共有による不変・不可逆な同盟ブロック、陣営内の協調・分業・規格統合、終局の戦後秩序分配。
* **[concept_frontier_dissidents_and_territorial_claims.md](./discussion/diplomacy_governance/concept_frontier_dissidents_and_territorial_claims.md)**
  * 自発的失踪者（民生ベース自活・こなれた武装の第三極コミュニティ）の社会・軍事生態、および自然国境と実効支配に基づく段階的領土要求システム。
* **[concept_occupation_governance_and_divided_economy.md](./discussion/diplomacy_governance/concept_occupation_governance_and_divided_economy.md)**
  * 敵都市・領土の占領統治方式（軍政・傀儡政権・直接併合）、治安度維持、分断された経済圏の管理。
* **[concept_capitulation_remnants_and_supply_chain.md](./discussion/diplomacy_governance/concept_capitulation_remnants_and_supply_chain.md)**
  * 敗戦国の無条件/条件付き降伏、残存武装勢力（レジスタンス/軍閥）のゲリラ化とサプライチェーンの寸断・再編。
* **[concept_factions_detail_and_naming.md](./discussion/diplomacy_governance/concept_factions_detail_and_naming.md)**
  * **架空6大国家の詳細設定・初期産業ブートローダーおよび命名プロトコル仕様書**。
  * 帝国A、大公国B、連邦C、共和国D、共同体E、連合Fの深層社会設定、統治執政機関、初期ブートローダー産業と得意モジュール。
  * 国家元首・前線最高司令官・主力兵器廠の公文書呼称体系、および各国の兵器型番命名プロトコル（Designator Standard）。
  * 降着惑星における初期外交関係マトリクスと地政学的緊張要因（主権独立と母星不介入の確定）。

### ⑤ 世界観・人口・進行フェーズ（`world_progression/`）

舞台となる惑星環境、初期開拓フェーズ、人口増殖、およびゲーム全体の進行推移・勝利条件。

* **[concept_frontier_civilization_and_ecosystem_dynamics.md](./discussion/world_progression/concept_frontier_civilization_and_ecosystem_dynamics.md)**
  * 惑星開拓初期の低文明・現地適応設定、原住エイリアンや現地植物の生態サイクル・環境変化に対する動的反応。
* **[concept_demographics_ectogenesis_and_speciation.md](./discussion/world_progression/concept_demographics_ectogenesis_and_speciation.md)**
  * **人口動態・人工子宮（エクトジェネシス）と種分化（適応進化）仕様書**。
  * 100年の時間差と人口爆発、人工子宮プラントの稼働、環境適応・遺伝子改変による人類の分岐・種分化。
* **[concept_environmental_dynamics_and_future_energy.md](./discussion/world_progression/concept_environmental_dynamics_and_future_energy.md)**
  * **環境保護・生態系フィードバックと未来エネルギー・熱量管理仕様検討書**。
  * タイル熱収支方程式、熱過負荷（機器停止・人工スーパーセル誘発）ペナルティ。
  * 海水電解グリーン水素・核融合（重水素濃縮）・海洋温度差発電（OTEC）・地熱発電のエネルギー体系。
  * 3大環境負荷インジケーター（熱・化学/重金属・生息域破壊）とガイア・リアクション（好熱性害虫暴走・酸性嵐）。
* **[concept_tectonics_volcanism_and_earthquake.md](./discussion/world_progression/concept_tectonics_volcanism_and_earthquake.md)**
  * **火山・地震災害およびプレートテクトニクス仕様検討書**。
  * プレート境界（収束・発散・トランスフォーム）主体のマクロ設計と断層の抽象化（歪み蓄積モデル）。
  * 火山灰雲（太陽光激減・航空停止）、溶岩流（サブタイル融解）、巨大地震・津波と海洋工法耐性（メガフロート/GBS）。
  * **防潮堤・海岸堤防工学体系**: 規模（小型・大型・地嵩上げ式スーパー堤防）× 断面形状（緩傾斜・階段・垂直壁・湾曲型）の流体力学・越波特性と都市利用。
  * **ゲーム全体の倫理観体系と地殻介入**: ゲームを貫く3大倫理軸（生存至上 vs ガイア共生 vs 工学至上）の激突点としての防災工学（命の選別、心理的閉塞、ガイア反発、地震誘発の外交危機）。
  * **現実の自然災害をゲーム化することの倫理的配慮と設計規律**: 実名・実被害の模倣排除、理不尽なランダム全壊（即死）の否定と予兆・観測・防災努力の尊重、センセーショナルな悲惨さ演出の排除、自然を単なる敵ではなく「惑星の息吹（恩恵と畏怖）」として昇華する制作姿勢。
  * 災害の二面性（超臨界地熱発電、マグマ性レアメタル濃集帯、肥沃土壌）と防災工学技術ツリー（減圧掘削・人工微小地震）。
* **[concept_technology_tree_structure.md](./discussion/world_progression/concept_technology_tree_structure.md)**
  * **研究開発・技術ウェブ（Tech Web）およびアフィニティ進化設計仕様書**。
  * 放射状（Radial）Web構造、幹技術（Trunk）と葉技術（Leaf）の階層構造、民生・産業進化と連動した技術発展。
  * 3大アフィニティ（Purity:純血地球化 / Harmony:調和共生 / Supremacy:超越自律）とハイブリッド混成進化（生体サイバネ・強化生態・自動重火工）。
  * 現場起因の自律研究ベクトル（Research Momentum）と国家指導者によるステアリング（重点特許・奨励金・移行ラグ）。
  * 戦場残骸の法医学的精査（Forensics）による技術ブースト（リバースエンジニアリング）とサプライチェーン制約。
* **[concept_game_progression_and_victory.md](./discussion/world_progression/concept_game_progression_and_victory.md)**
  * ゲーム進行の3大フェーズ推移（第1期:降着生存 → 第2期:領土画定・冷戦 → 第3期:総力戦・クライシス）、および5大勝利条件の設計、クリア後継続プレイ保証。
* **[concept_alien_lifecycle_and_nest_mechanics.md](./discussion/world_progression/concept_alien_lifecycle_and_nest_mechanics.md)**
  * **エイリアン生態ライフサイクル・巣窟運用および環境インタラクション仕様書**。
  * 巣窟の5大フェーズ（休眠/活性/繁殖/暴走スウォーム/枯渇）、瘴気（ミズマ）の動的変容と流体拡散・浄化・環境適応。
  * フェロモン誘導兵器による他国タイルへの意図的誘導（間接戦争）と、共生採取協定（高品位バイオ原質採取と破滅リスク）。

### ⑥ イベントシステム（`events/`）

ゲーム進行中に発生する動的イベント。

* **[concept_event_situational.md](./discussion/events/concept_event_situational.md)**
  * プレイヤーの特定行動にトリガーされて発生するシチュエーショナル・イベント。
* **[concept_event_stochastic.md](./discussion/events/concept_event_stochastic.md)**
  * 気象激変、地殻変動、エイリアンの異常活性化など、確率的・ランダムに発生する環境イベント。
* **[concept_event_catalog_and_chains.md](./discussion/events/concept_event_catalog_and_chains.md)**
  * イベントの8大ジャンル体系、現場エンティティ結合型の具体的イベントカタログ、不可逆な連鎖イベント（Event Chains）の設計。
* **[concept_faction_consensus_and_standardization.md](./discussion/events/concept_faction_consensus_and_standardization.md)**
  * 空気感による完全合意吸収、相手国承認に基づく平和的タイル移管、および通信非対応兵器の配備不可と3大調達アプローチ。
* **[concept_diplomatic_incidents_and_crises.md](./discussion/events/concept_diplomatic_incidents_and_crises.md)**
  * **外交危機・偶発紛争およびエスカレーション・ラダー設計書**。
  * 平時と戦時の間のグレーゾーン事態、危機段階メーター（Lv 0〜5）、民間船拿捕・亡命・回廊封鎖インシデント。
  * 第三極（自発的失踪者）を介した偽装代理紛争（ブラックオプス）と、開戦事由（Casus Belli）が戦後講和・吸収へ与える影響。

### ⑦ UI・情報閲覧システム（`ui_wiki/`）

情報表示とUX設計。

* **[concept_in_game_wiki.md](./discussion/ui_wiki/concept_in_game_wiki.md)**
  * ゲーム内百科事典（Civopedia相当）の構造、調査・研究進捗に応じた動的記事アンロック。
* **[concept_viewer_separation_wiki_and_live.md](./discussion/ui_wiki/concept_viewer_separation_wiki_and_live.md)**
  * 静的情報（Wiki）とリアルタイム戦況・ユニット状態（Live Viewer）のUI分離・統合アプローチ。
* **[concept_doctrine_steering_ui.md](./discussion/ui_wiki/concept_doctrine_steering_ui.md)**
  * **国家ドクトリン・自動化ステアリングUI/UX仕様書**。
  * 指示せず誘導するダッシュボードレイアウト、兵器クラス別RFP要求仕様カード、ゴーストゲージによる移行ラグ可視化。
  * 現場ボトムアップ改修（Field Mod）の制式化承認トレイ、および方面軍・戦闘団の弾薬互換警告インスペクター。
