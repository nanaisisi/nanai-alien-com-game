# ドキュメント構成・目録 (Documentation Index)

本プロジェクト（Nanai Alien Com Game）における各種仕様書・検討資料の構成および索引です。

---

## 📁 全体ディレクトリ構造

```text
docs/
├── INDEX.md                                # 本ファイル（ドキュメント目録）
├── GAME_DESIGN.md                          # ゲーム企画・コア仕様設計書（マスター）
├── force_structure/                        # 編成思想・体系・運用原則（7件）
│   ├── concept_military_echelon_and_combat_groups.md # 軍事編成・部隊単位体系および常設増強師団概念仕様書
│   ├── concept_headquarters_and_attached_units.md # 本部管理中隊（HHC）および本部付補助諸兵科 仕様定義書
│   ├── concept_hierarchical_headquarters_integration.md # 各階層本部付き隊の相互連携・指揮統制関係検討書
│   ├── concept_combat_group_structure_and_rfp_cycle.md # 増強師団TO&E構造・RFP自動更新および現場改修仕様書
│   ├── COMBAT_GROUP_LOGISTICS_INTEGRATION.md # 増強師団（TO&E）× 兵站・装備更新統合データ仕様書 (SEC-01)
│   ├── concept_toe_feedback_and_cross_echelon_methodology.md # 部隊編成（TO&E）水平展開・フィードバック手法および共通編制評価基準書
│   └── concept_corps_and_division_echelons.md # 軍団・師団・戦闘団 3層エシュロンおよび常設軍団制 仕様書
├── toe_definitions/                        # 具体的部隊編成（TO&E）定義（8件）
│   ├── toe_combat_group_heavy_armored.md   # 常設増強機甲師団 詳細編成・運用仕様書
│   ├── toe_combat_group_variants.md        # 常設増強師団（TO&E）バリエーション編成定義・目録
│   ├── toe_combat_group_airmobile.md       # 空中機動・VTOL空地強襲師団 詳細編成仕様書
│   ├── toe_combat_group_drone_swarm.md     # 無人自律・ドローン飽和特務師団 詳細編成仕様書
│   ├── toe_combat_group_siege_division.md  # 重装甲要塞突破師団 詳細編成仕様書
│   ├── toe_combat_group_mountain_jaeger.md # 山岳猟兵・隘路工兵師団 詳細編成仕様書
│   ├── toe_combat_group_bipedal_assault.md # 高級装甲・二脚歩行混成師団 詳細編成仕様書
│   ├── toe_combat_group_wheeled_reaction.md # 装輪高速機動師団 詳細編成仕様書
│   └── toe_combat_group_regional_defense.md # 地域配備・警備師団 詳細編成仕様書
├── specs/                                  # 個別兵器・システム確定・基本仕様書（4件）
│   ├── Faction.md                          # 勢力一覧・固有ボーナス/特性定義
│   ├── RESOURCE_ECONOMY_SYSTEM.md          # 資源・経済・サプライチェーン体系目録
│   ├── AVIATION_ASSETS_AND_DOCTRINE.md     # 師団直協航空アセット（米軍式航空連隊）運用仕様書
│   └── HIGH_TIER_BIPEDAL_COMBAT_WALKER.md  # 二脚歩行戦闘車両（高級装甲部隊）仕様書
├── tech/                                   # 技術検討・アーキテクチャ仕様・実装設計
│   ├── SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md # 全体実装依存関係・アーキテクチャ仕様書
│   ├── BEVY_0_20_INTEGRATION_REVIEW.md     # Bevy 0.20 仕様照合・技術検討書
│   └── RESOURCE_IMPLEMENTATION_BRIDGE.md   # 資源系 実装橋渡し案
├── planning/                               # 開発計画・ロードマップ・課題管理
│   ├── MISSING_DOCUMENTS_ROADMAP.md        # 未作成・不足ドキュメント体系と開発ロードマップ
│   ├── PENDING_ISSUES.md                   # 未確定・検討課題まとめ
│   ├── SECONDARY_MISSING_SPECS.md          # 二次的不足ドキュメント・システム統合仕様書体系
│   └── SECONDARY_SPECS_EVALUATION.md       # 2次的不足仕様・検討論点と実装アプローチ評価書
└── discussion/                             # 詳細設計・発展仕様・アイデア検討ドキュメント群（計49件）
    ├── automation/                         # 自動化システム（2件）
    │   ├── concept_automation_systems.md
    │   └── concept_autonomous_control_and_atmospheric_consensus.md
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
    ├── industry_logistics/                 # 産業・サプライチェーン・組織（9件）
    │   ├── concept_resource_economy_and_logistics.md
    │   ├── concept_industry_structure.md
    │   ├── concept_industry_dynamics_novelty_continuity.md
    │   ├── concept_industrial_evolution_and_supply_chains.md
    │   ├── concept_bootloader_and_national_supply_chains.md
    │   ├── concept_industry_module_mapping.md
    │   ├── concept_doctrine_and_equipment_steering.md
    │   ├── concept_transport_windows_and_infrastructure_bottlenecks.md
    │   └── concept_theater_operations_and_joint_sectors.md
    ├── map_tile/                           # マップ・サブタイル・領土拡張（7件）
    │   ├── sub_tile_seamless_connection.md
    │   ├── future_concept_subtile_industry.md
    │   ├── concept_tile_centric_expansion.md
    │   ├── concept_tile_progression_system.md
    │   ├── concept_maritime_governance_and_ocean_engineering.md
    │   ├── concept_subtile_combat_and_tactical_rules.md
    │   └── concept_infrastructure_grid_and_transit_artery.md
    ├── military_tactics/                   # 軍事・兵器モジュール・戦術運用（12件）
    │   ├── concept_weapon_module_commonality.md
    │   ├── example_weapon_modules_cases.md
    │   ├── concept_ammunition_caliber_standards_c.md
    │   ├── concept_future_legged_unmanned_rules.md
    │   ├── concept_ethics_autonomous_and_surrender_dynamics.md
    │   ├── concept_salvage_scuttling_and_compatibility.md
    │   ├── concept_dynamic_arsenal_geospatial.md
    │   ├── concept_frontier_industry_and_weapon_generations.md
    │   ├── concept_hidden_stacks_and_reserve_arsenal.md
    │   ├── concept_non_unit_sea_and_vehicle_transport.md
    │   ├── concept_homeworld_roadmaps_and_niche_industries.md
    │   └── concept_starter_modules_and_common_chassis_catalog.md
    ├── ui_wiki/                            # UI・情報閲覧システム（5件）
    │   ├── concept_in_game_wiki.md
    │   ├── concept_viewer_separation_wiki_and_live.md
    │   ├── concept_doctrine_steering_ui.md
    │   ├── concept_notification_system_and_presets.md
    │   └── concept_directive_manual_control_and_intent_ui.md
    └── world_progression/                  # 世界観・生態系・進行フェーズ（7件）
        ├── concept_frontier_civilization_and_ecosystem_dynamics.md
        ├── concept_demographics_ectogenesis_and_speciation.md
        ├── concept_environmental_dynamics_and_future_energy.md
        ├── concept_tectonics_volcanism_and_earthquake.md
        ├── concept_technology_tree_structure.md
        ├── concept_game_progression_and_victory.md
        └── concept_alien_lifecycle_and_nest_mechanics.md
```

---

## 📑 1. ゲーム企画・仕様書（`GAME_DESIGN.md`, `specs/`）

* **[GAME_DESIGN.md](./GAME_DESIGN.md)**：ゲーム企画・基本仕様設計書（マスタードキュメント）
  * ゲームコンセプト（4X SFターン制戦略、Civ:BEライク）
  * 全体マップ（大戦略層）と単一タイルマップ（戦術層）の二重スケール構造
  * 固定編成・戦闘団制（増強師団相当）による小部隊運用
  * 原住エイリアンとの共存／支配／排除の関係性システム
  * 技術ツリー、内政・資源、外交の基本方針
* **[specs/RESOURCE_ECONOMY_SYSTEM.md](./specs/RESOURCE_ECONOMY_SYSTEM.md)**：資源・経済・サプライチェーン体系目録（インデックス）
  * 一次資源（4種）・二次中間財・兵站消費財・食料・弾薬の加工流通フロー図
  * 産業構造、ブートローダー工廠、弾薬C規格、動脈インフラに関するドキュメント体系リンク集
  * Rust/Bevy実装用データ型マッピング
* **[specs/Faction.md](./specs/Faction.md)**：登場勢力設定仕様書
  * 母星の歴史・大戦停戦を経て入植した架空の6大国家概念（帝国A、大公国B、連邦C、共和国D、共同体E、連合F）
  * 各国の体制・文化圏背景、母星と入植惑星での外交関係（具体的な国名は未確定・継続検討事項）
  * アフィニティ（思想・適応路線）や経済/軍事ボーナスの基本方向性
  * ゲーム実装用データ: [`../assets/data/factions.csv`](../assets/data/factions.csv), [`../assets/data/city_names.csv`](../assets/data/city_names.csv)（勢力マスター＆全192都市名リスト）
* **[specs/AVIATION_ASSETS_AND_DOCTRINE.md](./specs/AVIATION_ASSETS_AND_DOCTRINE.md)**：師団直協航空アセット（戦闘・輸送ヘリ）運用仕様書
  * 師団直協航空連隊（計32機: 戦闘飛行隊×2＋輸送飛行隊×2、1個飛行隊＝8機）の機体スペック・役割定義。
  * CAS（近接航空支援）の一撃離脱・滞空時間制限、対空火網との相互作用。
  * 空中機動・兵站架橋（ヘリボーン、空中物資補給、MEDEVAC）、航空推進剤の消費ダイナミクス。
* **[specs/HIGH_TIER_BIPEDAL_COMBAT_WALKER.md](./specs/HIGH_TIER_BIPEDAL_COMBAT_WALKER.md)**：二脚歩行戦闘車両（高級装甲部隊）仕様書
  * ガングリフォン風の重厚・実戦的な二脚歩行兵器仕様（完全歩行駆動、ローラーダッシュ完全排除）。
  * 機体寸法ダイナミクス：大型戦略輸送機カーゴベイ規格限界（全高3.76m/全幅5.48m） vs 通常稼働・戦闘展開時（全高5〜6m/全幅6.5〜7.5m）。
  * 地上一般インフラの完全拒絶（道路・橋梁・トンネル破壊）と滑走路空輸展開モデル。
  * 通常重歩行型と短時間立体跳躍型（30〜60秒）の2大バリエーションおよび機甲戦闘団内の混成中隊（16両）配備。

---

## 🎖️ 2. 部隊編成思想・体系（`force_structure/`）

軍事エシュロン階層、2進数倍々補完（2-2構成）、国家RFPサイクル、兵站統合モデルを統括する思想・原則文書群。

* **[force_structure/concept_military_echelon_and_combat_groups.md](./force_structure/concept_military_echelon_and_combat_groups.md)**：軍事編成・部隊単位体系および戦闘団概念仕様書
  * 小隊（4両/機）・中隊（16両/機）・大隊（64両/機）・連隊・師団（戦闘団）の2進数・倍々構造（$2^N$ バイナリ・バディシステム）。
  * 地上打撃力と直協航空連隊（計32機: 1飛行隊8機×4）を有機的に内包する空地統合戦闘団の基本構造。
  * ゲーム的スロット枠概念を排した「部隊単位コレクション（`Vec<SubUnit>`）」設計。
  * 「地域配備（インフラ警備・害獣対処・低コスト）」から「機動運用（迅速転地・防衛力維持）」への戦略移行と留守警備隊メカニクス。
* **[force_structure/concept_headquarters_and_attached_units.md](./force_structure/concept_headquarters_and_attached_units.md)**：本部管理中隊（HHC）および本部付補助諸兵科 仕様定義書
  * 全部隊共通の「基盤的補助諸兵科（C4I指揮、通信、補給、衛生・除染、野戦整備・ARV、司令部警護）」と、部隊特性に応じた「本部付き専門部隊（戦闘工兵・架橋、重迫撃砲、対戦車、EW、JTAC等）」の2層構造。
  * 大隊・連隊・師団のエシュロン別HHC規模基準と、C4Iキルチェーン、共食い整備回収率、自活補給バッファへのゲームメカニクス連携。
* **[force_structure/concept_hierarchical_headquarters_integration.md](./force_structure/concept_hierarchical_headquarters_integration.md)**：各階層本部付き隊の相互連携・指揮統制関係検討書
  * 「師団司令部付隊 ⇄ 連隊本部管理中隊 ⇄ 大隊本部管理班」の3階層本部付き隊の役割分担と上下パイプライン。
  * 「火力は前線にいくらあっても良いが、非火力支援（工兵・重整備・大規模補給）は大隊から連隊・師団へ集約する」思想と、全階層で絶対に削減できない「C4I（キルチェーン）」および「衛生（対酸・生体毒のプラチナ・テン）」の不可欠性。
  * 兵站・回収の3段列構造（即時牽引 ➔ 前方支援点/共食い整備 ➔ 師団工作ドック）、施設工兵連携、および司令部被弾時の縮退・自律指揮権委譲ルールとECSデータ設計案。
* **[force_structure/concept_combat_group_structure_and_rfp_cycle.md](./force_structure/concept_combat_group_structure_and_rfp_cycle.md)**：戦闘団TO&E構造・RFP自動更新および現場改修仕様書
  * 標準諸兵科連合師団（4連隊基幹：戦車256両＋随伴重IFV256両の完全同数ペア）の代表的戦闘序列（TO&E）。
  * 国家RFP指示を受けた国内工廠の自動ブロック更新（Block I → II）と金型転換ラグ。
  * 過酷実戦環境が生むボトムアップ現場改修（Field Mod）と国家制式採用承認イベント。
  * 新旧混在による過渡期稼働率ペナルティ算出モデルと計画ロット更新の重要性。
* **[force_structure/COMBAT_GROUP_LOGISTICS_INTEGRATION.md](./force_structure/COMBAT_GROUP_LOGISTICS_INTEGRATION.md)**：戦闘団（TO&E）× 兵站・装備更新統合データ仕様書 (SEC-01)
  * インフラ結合度に応じた自動補給、兵站断絶時の稼働率低下と状態遷移（Normal ➔ Strained ➔ Critical ➔ Exhausted）。
  * 現場判断による共食い整備（Cannibalization）のステートマシン（ドナー部位の状態による劣化分岐ロジック）。
  * 直協航空連隊（計32機: 戦闘16機・輸送16機）の航空推進剤・補給係数。
* **[force_structure/concept_toe_feedback_and_cross_echelon_methodology.md](./force_structure/concept_toe_feedback_and_cross_echelon_methodology.md)**：部隊編成（TO&E）水平展開・フィードバック手法および共通編制評価基準書
  * 重機甲師団で確立された編成基準（2-2バイナリ、大隊付自走重迫16両集約、左右両翼スクリーン、突破と掃討の分離、3階層HHC、プラットフォーム二分化）を、他アーキタイプへ等価適用するための6大共通評価軸（チェックリスト）。
  * 空中機動・ドローン特務・要塞突破・山岳猟兵・二脚混成・装輪機動師団に対する診断・改定マトリクスと更新プロトコル。
* **[force_structure/concept_corps_and_division_echelons.md](./force_structure/concept_corps_and_division_echelons.md)**：常設機甲軍団の編成規模・最小構成検討書
  * 戦闘機動8連隊（約30,000名、戦闘車両1,520両）が「4個戦闘連隊基幹の2個師団を統合した【軍団（Corps）としての最小構成（Minimum Viable Corps）】」であるという構造的再定義。
  * 軍団（2〜4個師団の作戦スタック・直轄大深度砲兵/航空保有） ⇄ 師団（大戦略標準コマ・4個連隊諸兵科連合） ⇄ 戦闘団（サブタイル343マスの実動大隊TF）の3層関係と、最小構成（2個師団）の限界・異種混成軍団への発展検討論点。

---

## 🛡️ 3. 具体的部隊編成（TO&E）（`toe_definitions/`）

各諸兵科連合戦闘団バリエーションの詳細編成表（TO&E: Table of Organization and Equipment）および戦闘序列。

* **[toe_definitions/toe_combat_group_heavy_armored.md](./toe_definitions/toe_combat_group_heavy_armored.md)**：常設増強機甲師団 詳細編成・運用仕様書
  * 戦闘機動8連隊制（4個諸兵科連合機甲連隊［戦車256両＋重IFV256両＋迫撃128両］ ＋ 2個装甲化歩兵連隊［重IFV128両＋装甲APC128両＋迫撃64両］ ＋ 左右両翼偵察戦闘連隊［160両: 迫撃32両含］）。
  * 突破打撃軸（機甲4連隊）と掃討・要塞化軸（歩兵2連隊）がそれぞれ完全な2-2相互躍進（Leapfrogging）を遂行。各大隊付120C自走重迫撃砲中隊（計224両: 16両×14大隊）、独立特科・高射連隊、米軍式直協航空連隊（32機）。
* **[toe_definitions/toe_combat_group_variants.md](./toe_definitions/toe_combat_group_variants.md)**：常設増強師団（TO&E）バリエーション編成定義・目録
  * 2進数倍々・エシュロン体系（小隊4 ➔ 中隊16 ➔ 大隊64/80 ➔ 連隊 ➔ 師団）に基づくTO&E基本モデル。
  * 師団直協航空連隊（計32機: 戦闘飛行隊×2＋輸送飛行隊×2、1個飛行隊＝8機編成）および無人群編制。
  * 各種任務・ドクトリン別個別仕様書へのインデックス。
* **[toe_definitions/toe_combat_group_airmobile.md](./toe_definitions/toe_combat_group_airmobile.md)**：空中機動・VTOL空地強襲師団 詳細編成仕様書
  * 重武装戦闘VTOL 32機、強襲輸送VTOL 32機、空挺軽IFV 128両、大隊付120C超軽量自走迫 64両、FARP先行展開。
* **[toe_definitions/toe_combat_group_drone_swarm.md](./toe_definitions/toe_combat_group_drone_swarm.md)**：無人自律・ドローン飽和特務師団 詳細編成仕様書
  * 無人Std-T4車台共通化（突撃UGV 128両＋無人IFV 128両＋無人迫撃 64両）、有人統制装甲歩兵連隊、FPV母車連隊 64両。
* **[toe_definitions/toe_combat_group_siege_division.md](./toe_definitions/toe_combat_group_siege_division.md)**：重装甲要塞突破師団 詳細編成仕様書
  * 超重機甲2連隊（超重戦車128両）＋重装甲工兵・攻城歩兵2連隊の2-2均等構成、210C超重自走砲 32両、左右両翼スクリーン 160両。
* **[toe_definitions/toe_combat_group_mountain_jaeger.md](./toe_definitions/toe_combat_group_mountain_jaeger.md)**：山岳猟兵・隘路工兵師団 詳細編成仕様書
  * 下車登攀歩兵、迫撃砲多層分散（小隊60C/中隊90C/大隊120Cアンカー座）、多脚荷役機（Std-L4）64機、4脚斥候犬 64機。
* **[toe_definitions/toe_combat_group_bipedal_assault.md](./toe_definitions/toe_combat_group_bipedal_assault.md)**：高級装甲・二脚歩行混成師団 詳細編成仕様書
  * 戦車と通常二脚（64機）の小隊混成、短時間飛翔二脚（64機）による要衝跳躍急襲、戦略空輸展開、左右両翼スクリーン 160両。
* **[toe_definitions/toe_combat_group_wheeled_reaction.md](./toe_definitions/toe_combat_group_wheeled_reaction.md)**：装輪高速機動師団 詳細編成仕様書
  * 8x8装輪単一ファミリー共通化（共通化率95%）、装輪打撃2連隊（105C/25C）＋装輪歩兵2連隊（IFV/APC）、舗装動脈高速自走。
* **[toe_definitions/toe_combat_group_regional_defense.md](./toe_definitions/toe_combat_group_regional_defense.md)**：地域配備・警備師団 詳細編成仕様書
  * 管区常駐・現地インフラ自活（超低維持費）、留守警備隊基盤、旧式装甲車・牽引式120C重迫（大隊16門）とトーチカ複合防御。

---

## 🛠️ 4. 技術設計・アーキテクチャ（`tech/`）

* **[tech/SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md](./tech/SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md)**：全体実装依存関係・アーキテクチャ仕様書
  * システム全体の6層アーキテクチャ（エンジン・空間・経済・軍事・動態・UI）、モジュール間依存関係マトリクス、およびボトムアップ実装ロードマップ。
* **[tech/BEVY_0_20_INTEGRATION_REVIEW.md](./tech/BEVY_0_20_INTEGRATION_REVIEW.md)**：Bevy 0.20 仕様照合・技術検討書
  * Bevy 0.20 新機能（Weak System Ordering、`Val::Em`/`Rem`、`FixedNode`、WESL、標準カメラ等）と現行実装の適合状況、およびUIレスポンシブ化・並列化等の拡張ロードマップ。
* **[tech/RESOURCE_IMPLEMENTATION_BRIDGE.md](./tech/RESOURCE_IMPLEMENTATION_BRIDGE.md)**：資源系 実装橋渡し案
  * 資源経済設計とBevy実装コードを繋ぐ暫定ブリッジ仕様（データ構造、タイル収支、段階的実装フェーズ計画）。

---

## 📋 3. 開発計画・ロードマップ・課題（`planning/`）

* **[planning/PENDING_ISSUES.md](./planning/PENDING_ISSUES.md)**：未確定・検討課題まとめ（論点整理）
  * 歩行兵器の定義と不便さの具現化、国家概念の肉付け、戦闘団×モジュール共通化の接続など、掘り下げ不足な論点と今後の検討課題一覧。
* **[planning/MISSING_DOCUMENTS_ROADMAP.md](./planning/MISSING_DOCUMENTS_ROADMAP.md)**：未作成・不足ドキュメント体系と開発ロードマップ
  * 全体ドキュメントの最新成熟度マトリクス、明確に不足している5大領域（資源品目定義、技術ツリー、外交危機、エイリアン生態巣窟、ステアリングUI）、および開発ロードマップ。

---

## 💡 4. 詳細検討・発展仕様（`discussion/`）

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
* **[concept_infrastructure_grid_and_transit_artery.md](./discussion/map_tile/concept_infrastructure_grid_and_transit_artery.md)**
  * **大戦略インフラ・動脈網および中心サブタイル通過設計仕様書**。
  * サブタイルの複雑化（個別の電線・線路引き）を排し、大戦略親タイル間で交通網・電力網・通信網を管理する抽象化設計。
  * 高速道路・鉄道・リニアモーターカーの3段階交通網と、送電線・光ファイバー有線網の自動一体敷設。
  * 陸上ユニットがそのまま通過可能な「1タイル橋（海峡架橋）」、海洋ユニットが陸を渡る「複数タイル運河（地峡開削）」。
  * 冷却水と瘴気隔離を生かした「離島発電」、海底HVDC送電ケーブル網、および未電化・断絶沿岸を救済する「移動式給電船」。
  * 戦術マップ（343サブタイル）との接続：幹線インフラは必ず**「中心サブタイル」を貫通・交差**し、中央ハブや駅舎・橋頭堡・水門をめぐるドラマチックな攻防を提供。

### ② 軍事・兵器モジュール・戦術運用（`military_tactics/`）

ユニットの設計、兵器モジュール化、無人・自律兵器、戦場ルール。

* **[concept_weapon_module_commonality.md](./discussion/military_tactics/concept_weapon_module_commonality.md)**
  * 車体／シャシーと兵装モジュールの共通化設計（モジュラー兵器システム）。生産効率と改修自由度の両立。
* **[example_weapon_modules_cases.md](./discussion/military_tactics/example_weapon_modules_cases.md)**
  * 兵装モジュール・シャシー組み合わせの実装具体例（対空砲架、レールガン、ミサイルポッド、電子戦装備等）。
* **[concept_ammunition_caliber_standards_c.md](./discussion/military_tactics/concept_ammunition_caliber_standards_c.md)**
  * **弾薬・火砲規格体系（C規格 / 単位：C）仕様書**。
  * 母星文明の度量衡・工業規格において、弾丸の太さ（ボア径）を表す普遍単位「C（シー）」の成立と継承。
  * 英語略称ではなく単位記号そのものが「C」（1C ≒ 1mm相当のスケール感で直感性を担保）。
  * 完全整数ピッチの口径階層体系（6C/8C小火器、13C重機、15C/40C/180C電磁砲、25C〜50C機関砲、60C/80C/120C迫撃砲、120C〜140C戦車滑腔砲、160C/210C重火砲）。
  * 薬莢長表記を排したクラス・モジュラー方式、ケースレス（CT）・電熱化学（ETC）・電磁加速（Rail）の世代進化とサプライチェーン・兵站統合。
* **[concept_future_legged_unmanned_rules.md](./discussion/military_tactics/concept_future_legged_unmanned_rules.md)**
  * 多脚・二脚歩行兵器の特性検討（万能ではなく構造の複雑さ・整備負荷・脆弱性等の「不便さ」を伴うロマン枠としての議論深化）、無人ドローン・自律機械ユニットの階層（Tier）と通信途絶時の行動ルーチン。
* **[concept_ethics_autonomous_and_surrender_dynamics.md](./discussion/military_tactics/concept_ethics_autonomous_and_surrender_dynamics.md)**
  * 自律兵器運用における倫理的課題・国際条約、敵部隊の降伏判定と捕虜・無力化ユニットの処理。
* **[concept_salvage_scuttling_and_compatibility.md](./discussion/military_tactics/concept_salvage_scuttling_and_compatibility.md)**
  * 戦場での残骸回収（サルベージ）、鹵獲阻止のための自爆・自沈（スカットリング）、異文明・敵性技術モジュールの規格互換性とリバースエンジニアリング。
* **[concept_dynamic_arsenal_geospatial.md](./discussion/military_tactics/concept_dynamic_arsenal_geospatial.md)**
  * 地理・地形特性に応じた戦力展開、補給工廠（アーセナル）の空間的配置と動的兵站ラインの防衛。
* **[concept_frontier_industry_and_weapon_generations.md](./discussion/military_tactics/concept_frontier_industry_and_weapon_generations.md)**
  * **入植産業構造と母星人類兵器体系・世代進化仕様書**。
  * 母星の超精密ファブ・国際分業と、入植地の未熟なサプライチェーン・工作機械の落差が生む兵装の退行と再建プロセス。
  * 兵器世代の5段階体系（Gen 0:母星遺産ストック ──► Gen 1:開拓急造 ──► Gen 2:独立軍制式重工業 ──► Gen 3:母星基準復元 ──► Gen 4:異星超克アフィニティ特化）。
  * 複合装甲・ミリ波FCSから粗鋼多層溶接・光学測距へのダウンサイジング、新旧混在部隊における整備・補給ペナルティのゲーム力学。
  * ゲーム開始時の非資源消費型初期ユニット（探査バギー、治安警備、監視杭、共生体歩兵等）と虎の子機甲（Gen 0）の封印・戦略予備運用。
  * 装備の有限寿命（物理的減耗・不可逆劣化）vs 人間の訓練可能性（適応・学習）の非対称性。
* **[concept_hidden_stacks_and_reserve_arsenal.md](./discussion/military_tactics/concept_hidden_stacks_and_reserve_arsenal.md)**
  * **ユニット非表示スタック・非配備兵器および艦艇格納仕様書**。
  * 「1タイル1アクティブ駐留部隊」の原則を維持しつつ、地下要塞、輸送母艦、軍港、国家兵器庫（Arsenal）に部隊・兵装を多層格納する非表示スタック構造。
  * 艦艇・大型兵器のモスボール（保管役）メカニクス（維持費20%への圧縮、出渠・再活性化ラグ、港湾空襲リスク）。
  * 揚陸母艦・輸送列車への搭載スタックとサブタイル戦術戦闘（343マス）への実体化展開フロー、母艦被雷時の連鎖喪失リスク。
  * Bevy ECS（`DeploymentStatus`, `HangarBay`, `NationalArsenalPool`）データ構造とUIスタックピクトグラム表示。
* **[concept_non_unit_sea_and_vehicle_transport.md](./discussion/military_tactics/concept_non_unit_sea_and_vehicle_transport.md)**
  * **非ユニット輸送体系仕様書（非ユニット船舶海上輸送と徒歩部隊・車両輸送）**。
  * 「輸送船ユニットが海を埋め尽くす」欠陥を排除し、港湾間の海上回廊（シーレーン）および国家・港湾船腹プール（Sea Lift Capacity: SLC）を消費して陸上部隊を非ユニット航行させるシーリフト力学。
  * 哨戒艦艇・潜水艦網による通商破壊・航路安全度判定と、船団被襲撃による部隊損耗・水没沈没リスク。
  * 徒歩・軽装歩兵戦闘団による非戦闘車両・汎用トラック（GUV）・牽引トレーラーのアタッチ（モーターライズ化）。
  * 道路・平原での高速移動力向上と、下車戦闘ラグ、悪路進入不可、奇襲被弾時の脆弱性（防御力ペナルティ）の二律背反。
  * Bevy ECS（`SeaTransitRoute`, `PortSeaLiftCapability`, `UnitMobility`, `MobilityDoctrine`）設計。
* **[concept_homeworld_roadmaps_and_niche_industries.md](./discussion/military_tactics/concept_homeworld_roadmaps_and_niche_industries.md)**
  * **母星企業ロードマップ・現地重工と特化企業エコシステム仕様書**。
  * 母星企業が出発前に策定していた「段階的産業再建ロードマップ用モジュール規格」と、現地でそれを具現化・量産・改良するパワフルな「現地国策重工（背骨）」。
  * 自治体推進の産業振興と安価な無人化・民需転用キットを結ぶ「ハブ・コンソーシアム企業（血流）」。
  * 開拓農業・超深地採石・廃船解体・極地測量などの異業種からスピンオフし、現場の極限環境・エイリアン脅威に対処する「強烈なクセを持つ特化ニッチモジュール（スパイス）」。
  * 現場改修（Field Mod）と重工監査官の反発、産業振興イベントチェーンによる動的ドラマの調和。
* **[concept_starter_modules_and_common_chassis_catalog.md](./discussion/military_tactics/concept_starter_modules_and_common_chassis_catalog.md)**
  * **初期モジュール・基本シャシー共通カタログ仕様書 (Turn 0〜Gen 1)**。
  * スタート時の完全な平等性（競技的公平性）を担保する、全国家共通の母星標準規格（ISO-Terran Baseline）モジュール群。
  * 3大基本シャシー（装輪・装軌・多脚）の走破係数・積載スペック、基本武装・パワーパック・装甲モジュール定義。
  * Turn 0 初期配備戦力（Mk.0遠征戦車、探査バギー、治安警備小隊、受動監視杭）と初期ブートローダー工廠値。
  * 国家ごとの没入感を高めるフレーバー呼称マッピング（内部共通データと表示文字列の対応）。

### ③ 産業・サプライチェーン・組織運用（`industry_logistics/`）

生産ライン、モジュール製造、広域指揮統治および兵站輸送インフラ。

* **[concept_resource_economy_and_logistics.md](./discussion/industry_logistics/concept_resource_economy_and_logistics.md)**
  * **資源カテゴリ・物資格納・食料と弾薬規格の進化流通仕様書**。
  * 冷戦直後型母星共通規格から、同時並行開発による分化、後発追従・同盟協定・デファクトスタンダードによる再統合に至る弾薬規格の動的メカニクス。
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
* **[concept_transport_windows_and_infrastructure_bottlenecks.md](./discussion/industry_logistics/concept_transport_windows_and_infrastructure_bottlenecks.md)**
  * **輸送ウインドウ（タイムスロット）とインフラボトルネック検討仕様**。
  * 道路・鉄道・海上・航空・軌道の各輸送モードにおける時間枠・ノード/エッジ制約の構造的比較。
* **[concept_theater_operations_and_joint_sectors.md](./discussion/industry_logistics/concept_theater_operations_and_joint_sectors.md)**
  * **方面軍（Theater Command）システムと自動指揮・管理・占領統治**。
  * 専念領域（Dedicated Sector）と方面間共同管理領域（Joint Sector）の分離・連携メカニクス。

### ④ 自動化システム（`automation/`）

プレイヤーの操作負担を軽減し、前線と後方を結ぶ自律管理。

* **[concept_automation_systems.md](./discussion/automation/concept_automation_systems.md)**
  * 自動輸送ルート設定、定期哨戒（パトロール）、前線への自動補給ロジック。
* **[concept_autonomous_control_and_atmospheric_consensus.md](./discussion/automation/concept_autonomous_control_and_atmospheric_consensus.md)**
  * **自律制御アーキテクチャと集団合意ダイナミクス仕様書**。
  * 指導者（プレイヤー）の操作を「神の指」ではなく「社会の空気・暗黙の合意」として定義。
  * **過程と結果の分離**: 自動制御バランサーはUIを持たずバックグラウンドで平滑化に徹し、制御された結果は兵器カタログ・部隊移動力・弾薬混用率といった既存システム上に自然に表出。
  * 集団心理ゆえの「うっかり・行き過ぎ（極地で標準化車両立ち往生、流行への過剰陶酔）」の発生メカニクス。
  * LLMに一切依存しない、超軽量ルールベース判定＋システム間プロパティ反映による決定論的設計。

### ⑤ 外交・占領統治・戦後処理（`diplomacy_governance/`）

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
  * ゲーム実装用データ: [`../assets/data/factions.csv`](../assets/data/factions.csv), [`../assets/data/city_names.csv`](../assets/data/city_names.csv)

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
  * 3大アフィニティ（Purity:純血母星化 / Harmony:調和共生 / Supremacy:超越自律）とハイブリッド混成進化（生体サイバネ・強化生態・自動重火工）。
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
* **[concept_notification_system_and_presets.md](./discussion/ui_wiki/concept_notification_system_and_presets.md)**
  * **通知システム・プリセット仕様書**。
  * 自動制御が裏方で帳尻を合わせた結果（新モジュール誕生、代替ロット出荷、踏破デバフ、弾薬混用率等）をシステマチックに伝達。
  * プレイヤーの好みに合わせた通知プリセット機構（大局統帥／バランス型／現場ウォッチャー）およびユーザーカスタムプリセット保存機能。
  * トースト通知・チッカーログ・重要警告インジケータによる非侵入型の多層提示UI。
* **[concept_directive_manual_control_and_intent_ui.md](./discussion/ui_wiki/concept_directive_manual_control_and_intent_ui.md)**
  * **指針ベース手動操作・意図伝達UI/UX仕様書**。
  * 手続き（HOW）の強制入力を排し、大局的な意図（WHAT/GOAL）を直感的に伝える3大指針インターフェース（作戦ベクトル描画・要求仕様タグ指定・交戦姿勢トグル）。
  * 矢印を引くだけで道路トラック牽引・海上SLC消費・下車警戒隊形を自動選択する現場バランサー連携。
