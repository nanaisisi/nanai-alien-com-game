# ドキュメント構成・目録 (Documentation Index)

本プロジェクト（Nanai Alien Com Game）における各種仕様書・検討資料の構成および索引です。

---

## 📁 全体ディレクトリ構造

```text
docs/
├── INDEX.md                                # 本ファイル（全体ドキュメント目録）
├── GAME_DESIGN.md                          # ゲーム企画・コア仕様設計書（マスター）
├── units/                                  # 軍事ユニット・TO&E・装備・部隊体系総合（全62仕様書）
│   ├── INDEX_TOE.md                        # TO&Eマスターカテゴリ目録
│   ├── toe_indexes/                        # TO&Eカテゴリ別インデックス（全9件）
│   │   ├── ground_regular.md               # 陸上・正規軍編成インデックス
│   │   ├── ground_alien.md                 # 陸上・エイリアン軍団インデックス
│   │   ├── ground_multi_species.md         # 陸上・多種族協調編成インデックス
│   │   ├── air_regular.md                  # 航空・正規軍編成インデックス
│   │   ├── air_alien.md                    # 航空・エイリアン編隊インデックス
│   │   ├── air_multi_species.md            # 航空・多種族協調編成インデックス
│   │   ├── naval_regular.md                # 海上・正規軍編成インデックス
│   │   ├── naval_alien.md                  # 海上・エイリアン艦隊インデックス
│   │   └── naval_multi_species.md          # 海上・多種族協調編成インデックス
│   ├── toe/                                # 具体的部隊編成（TO&E）定義実体（全51仕様書）
│   │   ├── ground/                         # 陸上部隊編成（regular:13, alien:6, multi_species:1, fusion:1）
│   │   ├── naval/                          # 海上部隊編成（regular:6, alien:6, multi_species:1, fusion:1）
│   │   └── air/                            # 航空部隊編成（regular:14, alien:10, multi_species:1, fusion:1）
│   ├── equipment/                          # 兵器・装備カタログ仕様書（全6仕様書）
│   │   ├── ground/                         # 陸上装備（ground_regular_equipment, ground_alien_equipment）
│   │   ├── naval/                          # 海上装備（naval_regular_equipment, naval_alien_equipment）
│   │   └── air/                            # 航空装備（air_regular_equipment, air_alien_equipment）
│   ├── force/                              # 編成思想・エシュロン・運用原則（全10仕様書）
│   │   ├── concept_military_echelon_and_combat_groups.md # 軍事編成・部隊単位体系および戦闘団概念仕様書
│   │   ├── concept_corps_variants.md       # 常設増強軍団バリエーション編成定義書
│   │   ├── concept_headquarters_and_attached_units.md # 本部管理中隊（HHC）および本部付補助諸兵科仕様
│   │   ├── concept_hierarchical_headquarters_integration.md # 各階層本部付き隊の相互連携・指揮統制
│   │   ├── concept_combat_group_structure_and_rfp_cycle.md # 戦闘団TO&E構造・RFP自動更新および現場改修
│   │   ├── COMBAT_GROUP_LOGISTICS_INTEGRATION.md # 戦闘団×兵站・装備更新統合データ仕様書 (SEC-01)
│   │   ├── concept_toe_feedback_and_cross_echelon_methodology.md # TO&E水平展開・評価基準書
│   │   ├── concept_corps_and_division_echelons.md # 1ユニット＝2個師団1軍団制 概念・編制検討書
│   │   ├── concept_ground_vehicle_chassis_and_variants.md # 陸上装甲・戦闘車両総合仕様書
│   │   └── concept_personnel_and_manning_standards.md # 軍団・師団 人員規模および定員算出基準
│   └── specs/                              # 師団直協・高級装甲兵器仕様書（2件）
│       ├── AVIATION_ASSETS_AND_DOCTRINE.md # 師団直協航空アセット運用仕様書
│       └── HIGH_TIER_BIPEDAL_COMBAT_WALKER.md # 二脚歩行戦闘車両仕様書
├── specs/                                  # 共通システム確定・基本仕様書（5件）
│   ├── Faction.md                          # 勢力一覧・固有ボーナス/特性定義
│   ├── LAND_USE_SYSTEM.md                  # 土地利用・開発システム仕様書
│   ├── RESOURCE_ECONOMY_SYSTEM.md          # 資源・経済・サプライチェーン体系目録
│   ├── STEERING_UI_FLOW.md                 # 多層ステアリングUI画面遷移・操作フロー仕様書
│   └── WATER_AND_SALINITY_SYSTEM.md        # 水域・塩分濃度システム仕様書
├── tech/                                   # 技術検討・アーキテクチャ仕様・実装設計（4件）
│   ├── SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md # 全体実装依存関係・アーキテクチャ仕様書
│   ├── BEVY_0_20_INTEGRATION_REVIEW.md     # Bevy 0.20 仕様照合・技術検討書
│   ├── RESOURCE_IMPLEMENTATION_BRIDGE.md   # 資源系 実装橋渡し案
│   └── STARTUP_OPTIONS.md                  # 起動オプション・コマンドライン引数仕様書
├── planning/                               # 開発計画・ロードマップ・課題管理（7件）
│   ├── CONSISTENCY_AUDIT_PLAN.md           # 仕様・ドキュメント整合性チェック計画書（本計画）
│   ├── MISSING_DOCUMENTS_ROADMAP.md        # 未作成・不足ドキュメント体系と開発ロードマップ
│   ├── MISSING_SPECS_EXECUTION_PLAN.md     # 不足ドキュメント整備・具体化実行計画書
│   ├── PENDING_ISSUES.md                   # 未確定・検討課題まとめ
│   ├── SECONDARY_MISSING_SPECS.md          # 二次的不足ドキュメント・システム統合仕様書体系
│   ├── SECONDARY_SPECS_EVALUATION.md       # 2次的不足仕様・検討論点と実装アプローチ評価書
│   └── NEXT_PLAN_LAND_BATTLESHIP_AND_AIR_NAVAL_REORG.md # 次期設計計画：陸上戦艦・艦隊航空隊整理
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
    │   ├── concept_starter_modules_and_common_chassis_catalog.md
    │   └── concept_advanced_tech_dew_unmanned_and_naval_doctrine.md
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
* **[specs/WATER_AND_SALINITY_SYSTEM.md](./specs/WATER_AND_SALINITY_SYSTEM.md)**：水域・塩分濃度システム仕様書
  * 外洋（Ocean）と閉鎖水域（Lake）の水域グラフ連結成分（BFS）自動判定ロジック。
  * 実用塩分単位（PSU / ‰）に基づく4段階水質区分（淡水、汽水、海水、高塩分・塩湖）。
  * カスピ海型巨大内陸湖（汽水）や死海型乾燥塩湖（鉱物資源特化）の環境適応・基礎産出力・戦略的差別化。
  * 3Dマテリアル、ミニマップピクセル描画、およびHUD（PLANET SURVEYOR）への情報統合。
* **[specs/LAND_USE_SYSTEM.md](./specs/LAND_USE_SYSTEM.md)**：土地利用・開発システム仕様書
  * 脱・都市中心主義に基づくタイルの地目指定・区画整備（産業・採取・居住・軍事の4大カテゴリ）。
  * 地形・水系（河川・塩分濃度）と土地利用の適性マトリクスおよび淡水灌漑ボーナス。
  * Tier 0（未開地）からTier 4（要塞中枢複合）への段階的発展と、地目転換（解体・再整地ラグ、スクラップ回収、Tierリセット）。
  * 隣接シナジー（集積経済）と公害・エイリアン刺激フリクション、およびBevy ECS実装用データ構造（`LandZoning`, `TileDevelopment`）。
* **[units/specs/AVIATION_ASSETS_AND_DOCTRINE.md](./units/specs/AVIATION_ASSETS_AND_DOCTRINE.md)**：師団直協航空アセット（戦闘・輸送ヘリ）運用仕様書
  * 師団直協航空連隊（計32機: 戦闘飛行隊×2＋輸送飛行隊×2、1個飛行隊＝8機）の機体スペック・役割定義。
  * CAS（近接航空支援）の一撃離脱・滞空時間制限、対空火網との相互作用。
  * 空中機動・兵站架橋（ヘリボーン、空中物資補給、MEDEVAC）、航空推進剤の消費ダイナミクス。
* **[units/specs/HIGH_TIER_BIPEDAL_COMBAT_WALKER.md](./units/specs/HIGH_TIER_BIPEDAL_COMBAT_WALKER.md)**：二脚歩行戦闘車両（高級装甲部隊）仕様書
  * ガングリフォン風の重厚・実戦的な二脚歩行兵器仕様（完全歩行駆動、ローラーダッシュ完全排除）。
  * 機体寸法ダイナミクス：大型戦略輸送機カーゴベイ規格限界（全高3.76m/全幅5.48m） vs 通常稼働・戦闘展開時（全高5〜6m/全幅6.5〜7.5m）。
  * 地上一般インフラの完全拒絶（道路・橋梁・トンネル破壊）と滑走路空輸展開モデル。
  * 通常重歩行型と短時間立体跳躍型（30〜60秒）の2大バリエーションおよび機甲戦闘団内の混成中隊（16両）配備。

---

## 🎖️ 2. 部隊編成思想・体系（`units/force/`）

軍事エシュロン階層、2進数倍々補完（2-2構成）、国家RFPサイクル、兵站統合モデルを統括する思想・原則文書群。

* **[units/force/concept_military_echelon_and_combat_groups.md](./units/force/concept_military_echelon_and_combat_groups.md)**：軍事編成・部隊単位体系および戦闘団概念仕様書
  * 小隊（4両/機）・中隊（16両/機）・大隊（64両/機）・連隊・師団（戦闘団）の2進数・倍々構造（$2^N$ バイナリ・バディシステム）。
  * 地上打撃力と直協航空連隊（計32機: 1飛行隊8機×4）を有機的に内包する空地統合戦闘団の基本構造。
  * ゲーム的スロット枠概念を排した「部隊単位コレクション（`Vec<SubUnit>`）」設計。
  * 「地域配備（インフラ警備・害獣対処・低コスト）」から「機動運用（迅速転地・防衛力維持）」への戦略移行と留守警備隊メカニクス。
* **[units/force/concept_headquarters_and_attached_units.md](./units/force/concept_headquarters_and_attached_units.md)**：本部管理中隊（HHC）および本部付補助諸兵科 仕様定義書
  * 全部隊共通の「基盤的補助諸兵科（C4I指揮、通信、補給、衛生・除染、野戦整備・ARV、司令部警護）」と、部隊特性に応じた「本部付き専門部隊（戦闘工兵・架橋、重迫撃砲、対戦車、EW、JTAC等）」の2層構造。
  * 大隊・連隊・師団のエシュロン別HHC規模基準と、C4Iキルチェーン、共食い整備回収率、自活補給バッファへのゲームメカニクス連携。
* **[units/force/concept_hierarchical_headquarters_integration.md](./units/force/concept_hierarchical_headquarters_integration.md)**：各階層本部付き隊の相互連携・指揮統制関係検討書
  * 「師団司令部付隊 ⇄ 連隊本部管理中隊 ⇄ 大隊本部管理班」の3階層本部付き隊の役割分担と上下パイプライン。
  * 「火力は前線にいくらあっても良いが、非火力支援（工兵・重整備・大規模補給）は大隊から連隊・師団へ集約する」思想と、全階層で絶対に削減できない「C4I（キルチェーン）」および「衛生（対酸・生体毒のプラチナ・テン）」の不可欠性。
  * 兵站・回収の3段列構造（即時牽引 ➔ 前方支援点/共食い整備 ➔ 師団工作ドック）、施設工兵連携、および司令部被弾時の縮退・自律指揮権委譲ルールとECSデータ設計案。
* **[units/force/concept_combat_group_structure_and_rfp_cycle.md](./units/force/concept_combat_group_structure_and_rfp_cycle.md)**：戦闘団TO&E構造・RFP自動更新および現場改修仕様書
  * 標準諸兵科連合師団（4連隊基幹：戦車256両＋随伴重IFV256両の完全同数ペア）の代表的戦闘序列（TO&E）。
  * 国家RFP指示を受けた国内工廠の自動ブロック更新（Block I → II）と金型転換ラグ。
  * 過酷実戦環境が生むボトムアップ現場改修（Field Mod）と国家制式採用承認イベント。
  * 新旧混在による過渡期稼働率ペナルティ算出モデルと計画ロット更新の重要性。
* **[units/force/COMBAT_GROUP_LOGISTICS_INTEGRATION.md](./units/force/COMBAT_GROUP_LOGISTICS_INTEGRATION.md)**：戦闘団（TO&E）× 兵站・装備更新統合データ仕様書 (SEC-01)
  * インフラ結合度に応じた自動補給、兵站断絶時の稼働率低下と状態遷移（Normal ➔ Strained ➔ Critical ➔ Exhausted）。
  * 現場判断による共食い整備（Cannibalization）のステートマシン（ドナー部位の状態による劣化分岐ロジック）。
  * 直協航空連隊（計32機: 戦闘16機・輸送16機）の航空推進剤・補給係数。
* **[units/force/concept_toe_feedback_and_cross_echelon_methodology.md](./units/force/concept_toe_feedback_and_cross_echelon_methodology.md)**：部隊編成（TO&E）水平展開・フィードバック手法および共通編制評価基準書
  * 重機甲師団で確立された編成基準（2-2バイナリ、大隊付自走重迫16両集約、左右両翼スクリーン、突破と掃討の分離、3階層HHC、プラットフォーム二分化）を、他アーキタイプへ等価適用するための6大共通評価軸（チェックリスト）。
  * 空中機動・ドローン特務・要塞突破・山岳猟兵・二脚混成・装輪機動師団に対する診断・改定マトリクスと更新プロトコル。
* **[units/force/concept_corps_and_division_echelons.md](./units/force/concept_corps_and_division_echelons.md)**：1ユニット＝2個師団1軍団制 概念・編制検討書
  * 大戦略マップの1コマ（1ユニット）の実体が「各師団正面4連隊＋師団付特科連隊＋師団輸送飛行隊（8機）を持つ2個師団」からなる「1個軍団（Corps / 最小構成）」であるという構造的定義。
  * 軍団直属の集中航空戦力「軍団航空隊（4個飛行隊・計32機）」と師団自前の輸送飛行隊（8機×2）による2層航空運用。
* **[units/force/concept_ground_vehicle_chassis_and_variants.md](./units/force/concept_ground_vehicle_chassis_and_variants.md)**：陸上装甲・戦闘車両プラットフォームおよびバリアント総合仕様書
  * 陸上戦闘車両の5大車体系統（重装軌戦車系、装軌装甲車系、8輪装甲装輪車系、重装輪トラック系、4×4高機動系）の明確な階層区分。
  * 前線戦闘を担う「同一8輪装甲装輪車プラットフォーム」による10大バリアント共通化体系（IFV/APC/RCV/火力MCV/自走迫/指揮/患者輸送/CBRN/ARV/CEV）。
  * 重砲の2大マウント設計（160C全周旋回砲塔型 ⇄ 210C車体据付型）と、対空車両の3階層防空体系（自走対空戦車 ＋ 重装輪SAM ＋ 4×4近接SAM）。
* **[units/force/concept_corps_variants.md](./units/force/concept_corps_variants.md)**：常設増強軍団（TO&E）バリエーション編成定義書
  * 2進数倍々・エシュロン体系（小隊4 ➔ 中隊16 ➔ 大隊64/80 ➔ 連隊 ➔ 師団 ➔ 2個師団統合軍団）に基づくTO&E基本モデル。
  * **装甲化軍団（戦車2連隊＋装輪偵察2連隊＋装甲歩兵2連隊）**および**徒歩・自動車化歩兵多層エシュロン（Tier 1〜Tier 6）**の体系的定義。
  * 軍団直轄航空隊（計32機: 戦闘16＋輸送16）および各師団付輸送飛行隊（8機×2）、多層防空、戦車車体自走重砲体系。
* **[units/force/concept_personnel_and_manning_standards.md](./units/force/concept_personnel_and_manning_standards.md)**：軍団・師団 人員規模および定員算出基準仕様書
  * 下車歩兵一律固定8名分隊制（乗員＋8名分隊）と、主要戦闘車両（戦車3名、IFV11名、APC10名、RCV5名、MCV3名、迫撃4名、特科4名）の乗員基準。
  * 師団別・部隊別の積算根拠（1個師団 約14,000名、軍団直轄 約2,100名、大戦略1コマ総員 約30,100名）。
  * Tooth-to-Tail比率（直接戦闘員63% 対 後方支援員37%）と共食い整備・衛生後送による部隊抗堪性モデル。

---

## 🛡️ 3. 具体的部隊編成（TO&E）（`units/toe/`）

各諸兵科連合戦闘団バリエーションの詳細編成表（TO&E: Table of Organization and Equipment）および戦闘序列。作戦領域（陸・海・空）× 体系（regular, alien, multi_species, fusion）に分類。

### 陸上・人類通常軍団（`units/toe/ground/regular/`）

* **[units/toe/ground/regular/toe_corps_civilian_refugees.md](./units/toe/ground/regular/toe_corps_civilian_refugees.md)**：民間人・避難民群（Tier 0-A）詳細編成・運用仕様書
  * 非武装開拓民・避難民 約40,000名、民間バス・トラック・徒歩による非戦闘避難ユニット。護衛必須。
* **[units/toe/ground/regular/toe_corps_militia.md](./units/toe/ground/regular/toe_corps_militia.md)**：民兵軍団（Tier 0-B）詳細編成・運用仕様書
  * 民間自衛・治安維持部隊（総員 約32,000名、民間徴用トラック・テクニカル機銃車・旧式牽引榴弾砲）。
* **[units/toe/ground/regular/toe_corps_light_infantry.md](./units/toe/ground/regular/toe_corps_light_infantry.md)**：軽歩兵軍団（Tier 1）詳細編成仕様書
  * 車両進入不能な山岳・密林・洞窟を踏破する完全徒歩行軍軍団（総員 約24,000名、地上車両0両、軍団直轄・師団ヘリ計48機、分散迫撃砲ドクトリン）。
* **[units/toe/ground/regular/toe_corps_motorized_light_infantry.md](./units/toe/ground/regular/toe_corps_motorized_light_infantry.md)**：自動車化軽歩兵軍団（Tier 2）詳細編成仕様書
  * 4x4 LAV基幹の軽装甲化歩兵部隊（総員 約26,000名、牽引式155mm榴弾砲・陣地展開運用）。
* **[units/toe/ground/regular/toe_corps_motorized_infantry.md](./units/toe/ground/regular/toe_corps_motorized_infantry.md)**：自動車化歩兵軍団（Tier 3）詳細編成仕様書
  * 4x4 LAV ＋ トラック車載自走榴弾砲（T-SPH）による高度機動打撃部隊（総員 約28,000名、陣地進入即射・陣地変換）。
* **[units/toe/ground/regular/toe_corps_wheeled_mobile.md](./units/toe/ground/regular/toe_corps_wheeled_mobile.md)**：装輪機動軍団（Tier 4）詳細編成仕様書
  * 8x8装輪単一ファミリー共通化（共通化率95%）、装輪打撃連隊＋装輪装甲歩兵連隊、舗装動脈高速自走展開。
* **[units/toe/ground/regular/toe_corps_armored.md](./units/toe/ground/regular/toe_corps_armored.md)**：装甲化軍団（Tier 5）詳細編成仕様書
  * 主力戦車2個連隊（履帯）＋ 装輪装甲2個連隊（8x8）のハイブリッド諸兵科連合軍団。履帯戦車の突破力と装輪車の長駆展開力を両立。
* **[units/toe/ground/regular/toe_corps_heavy_armored.md](./units/toe/ground/regular/toe_corps_heavy_armored.md)**：常設増強機甲軍団（Tier 6）詳細編成・運用仕様書
  * 戦車4連隊基幹の2個師団統合型（戦車256両、重IFV384両、APC128両、120C自走迫224両、左右両翼スクリーン160両、航空機48機）。突破打撃軸（機甲4連隊）と掃討・要塞化軸（歩兵2連隊）が相互躍進を遂行。独立特科（160C/210C）・多層高射連隊。
* **[units/toe/ground/regular/toe_corps_airmobile.md](./units/toe/ground/regular/toe_corps_airmobile.md)**：空中機動・空地強襲軍団 詳細編成仕様書
  * 地上重戦車を排した空中機動2個師団統合運用。強襲航空旅団・軍団直轄航空打撃群（計176機）、空挺軽IFV、大隊付120C超軽量自走迫、FARP先行展開。
* **[units/toe/ground/regular/toe_corps_bipedal_assault.md](./units/toe/ground/regular/toe_corps_bipedal_assault.md)**：高級装甲・二脚歩行混成軍団 詳細編成仕様書
  * 戦車と通常二脚歩行機の混成、短時間飛翔二脚による要衝跳躍急襲、戦略空輸展開、左右両翼装輪スクリーン。
* **[units/toe/ground/regular/toe_corps_long_range_missile.md](./units/toe/ground/regular/toe_corps_long_range_missile.md)**：長距離ミサイル打撃軍団 詳細編成仕様書
  * 大戦略マップ上で2タイル先へスタンドオフ精密打撃を行うTEL重装輪発射機（128両）専門部隊。
* **[units/toe/ground/regular/toe_corps_land_battleship.md](./units/toe/ground/regular/toe_corps_land_battleship.md)**：陸上戦艦打撃軍団 詳細編成・運用仕様書
  * 超大型陸上戦艦（レールガン主砲）2隻を核とし、陸上空母・防空艦・近接護衛艦を配備した地上決戦機動要塞群。
* **[units/toe/ground/regular/toe_corps_science_survey.md](./units/toe/ground/regular/toe_corps_science_survey.md)**：調査・科学探査軍団 詳細編成仕様書
  * 有毒大気・極限環境を無効化し、異星遺跡・生体巣穴の深層解析と科学研究ボーナス獲得を担う特務探査部隊。

### 陸上・エイリアン軍団（野生・固有エイリアン）（`units/toe/ground/alien/`）

* **[units/toe/ground/alien/toe_alien_corps_small_swarm.md](./units/toe/ground/alien/toe_alien_corps_small_swarm.md)**：小型エイリアン軍団 詳細編成・生態仕様書
  * 数千〜数万頭の小型格闘種による軽量スウォーム軍団。山岳・密林・洞窟の不整地浸透、下車歩兵・非装甲部隊の殺傷に特化。
* **[units/toe/ground/alien/toe_alien_corps_medium_swarm.md](./units/toe/ground/alien/toe_alien_corps_medium_swarm.md)**：中型エイリアン軍団 詳細編成・生態仕様書
  * カマキリ状中型格闘種基幹。4x4軽装甲車（LAV）を短時間で解体・破壊し、装甲車（IFV/APC）には時間を掛けて装甲板をこじ開ける。戦車は破壊不能。
* **[units/toe/ground/alien/toe_alien_corps_heavy_tank.md](./units/toe/ground/alien/toe_alien_corps_heavy_tank.md)**：大型エイリアン軍団 詳細編成・生態仕様書
  * 戦車型（大型格闘種）基幹。主力戦車に組み付き、長い時間を掛けて装甲を切り裂き破壊する泥沼の決戦格闘軍団。
* **[units/toe/ground/alien/toe_alien_corps_colossus.md](./units/toe/ground/alien/toe_alien_corps_colossus.md)**：超大型エイリアン軍団 詳細編成・生態仕様書
  * 全高20m級の超大型接近戦型巨獣（タイタン級）を核とする要塞粉砕軍団。圧倒的な質量と超巨大鎌腕で敵戦車・防衛線・バンカーを踏み潰す。
* **[units/toe/ground/alien/toe_alien_corps_needle_artillery.md](./units/toe/ground/alien/toe_alien_corps_needle_artillery.md)**：長距離砲撃軍団 詳細編成・生態仕様書
  * 全高20m級の超大型ニードル砲撃種を核とする遠距離支援部隊。巨体の割にはミサイル程度の威力しかない巨大生体棘弾（ニードル）を放物線投射。
* **[units/toe/ground/alien/toe_alien_corps_aviation_melee.md](./units/toe/ground/alien/toe_alien_corps_aviation_melee.md)**：陸棲航空格闘軍団 詳細編成・生態仕様書
  * 有翼格闘種（ワイバーン型）が低空急降下し、地上部隊の頭上や車両上面（ルーフ）を爪と鎌腕で切り裂く地上急襲航空群（制空権確保は空棲種の担当）。

### 陸上・混成軍団および通常上位世代（Gen 4）

* **[units/toe/ground/multi_species/toe_corps_multi_species_ground.md](./units/toe/ground/multi_species/toe_corps_multi_species_ground.md)**：多種族混成陸上軍団（全11カテゴリー）詳細仕様書
  * 人類兵力50%と同盟エイリアン50%の共生協同運用、自己治癒（再生）および二重兵站ドクトリン。
* **[units/toe/ground/fusion/toe_corps_fusion_ground.md](./units/toe/ground/fusion/toe_corps_fusion_ground.md)**：通常陸上軍団・上位世代発展仕様書（生体技術・異星超克改修 / Gen 4）
  * 通常11カテゴリーの近代化改修。自己修復性生体甲殻、ハイブリッド駆動、神経リンクによる上位世代発展仕様。

### 海上戦闘団（`units/toe/naval/`）

#### 通常海上戦闘団（人類通常艦隊）（`units/toe/naval/regular/`）

* **[units/toe/naval/regular/toe_naval_fleet_battle.md](./units/toe/naval/regular/toe_naval_fleet_battle.md)**：通常艦隊 詳細編成・運用仕様書
  * 計62隻＋32機（小型空母2隻、防空駆逐艦4隻、汎用駆逐艦8隻、汎用フリゲート16隻、護衛コルベット32隻）。外洋水上打撃戦・制海権掌握の中核。
* **[units/toe/naval/regular/toe_naval_fleet_battleship.md](./units/toe/naval/regular/toe_naval_fleet_battleship.md)**：戦艦艦隊 詳細編成・運用仕様書
  * 計62隻（超弩級戦艦2隻、防空駆逐艦4隻、汎用駆逐艦8隻、汎用フリゲート16隻、護衛コルベット32隻）。計16門の`EM-180C`超大型電磁投射砲（レールガン）による大戦略2タイル先への超長距離沿岸砲撃。
* **[units/toe/naval/regular/toe_naval_fleet_carrier.md](./units/toe/naval/regular/toe_naval_fleet_carrier.md)**：航空艦隊 詳細編成・運用仕様書
  * 計62隻＋128機（大型正規空母2隻、防空駆逐艦4隻、汎用駆逐艦8隻、汎用フリゲート16隻、護衛コルベット32隻）。大戦略2タイル先への広域制空・洋上航空阻止・敵前上陸支援。
* **[units/toe/naval/regular/toe_naval_fleet_escort.md](./units/toe/naval/regular/toe_naval_fleet_escort.md)**：護衛艦隊 詳細編成・運用仕様書
  * 計28隻（汎用駆逐艦4隻、汎用フリゲート8隻、護衛コルベット16隻）。シーレーン防衛および民間輸送船団の専従護衛。
* **[units/toe/naval/regular/toe_naval_fleet_civilian.md](./units/toe/naval/regular/toe_naval_fleet_civilian.md)**：民間船艦隊 詳細編成・運用仕様書
  * 計32隻（大型コンテナ船8隻、中型貨物船16隻、沿岸連絡船8隻）。非武装の戦略資源・物資・食料の大量海上空輸・輸送船団。
* **[units/toe/naval/regular/toe_naval_fleet_armed_merchant.md](./units/toe/naval/regular/toe_naval_fleet_armed_merchant.md)**：武装船艦隊 詳細編成・運用仕様書
  * 計32隻（特設武装貨物船8隻、武装沿岸艇16隻、特設護衛船8隻）。民間船にコンテナ兵装・速射砲を仮設した開拓初期の自衛・警備船団。

#### 野生・固有エイリアン海洋群（エイリアン艦隊）（`units/toe/naval/alien/`）

* **[units/toe/naval/alien/toe_alien_fleet_patrol.md](./units/toe/naval/alien/toe_alien_fleet_patrol.md)**：エイリアン哨戒艦隊 詳細編成・生態仕様書
  * 小型刺胞遊泳個体、高速遊泳魚雷獣、表層監視種による俊敏な沿岸監視・警戒海洋群。
* **[units/toe/naval/alien/toe_alien_fleet_escort.md](./units/toe/naval/alien/toe_alien_fleet_escort.md)**：エイリアン護衛艦隊 詳細編成・生態仕様書
  * 棘皮防空個体、超音波迎撃種、中型護衛甲殻獣による防空・対潜水生体迎撃・遊泳路防衛海洋群。
* **[units/toe/naval/alien/toe_alien_fleet_regular.md](./units/toe/naval/alien/toe_alien_fleet_regular.md)**：エイリアン通常艦隊 詳細編成・生態仕様書
  * 中型重装甲甲殻獣、強酸投射種、生体魚雷射出種による外洋水上打撃・制海権掌握生体群。
* **[units/toe/naval/alien/toe_alien_fleet_battleship.md](./units/toe/naval/alien/toe_alien_fleet_battleship.md)**：エイリアン戦艦艦隊 詳細編成・生態仕様書
  * 全長200m超の超大型甲殻巨獣（リヴァイアサン級）、重キャビテーション衝撃砲種による決戦打撃および沿岸2タイルの生体砲撃破壊。
* **[units/toe/naval/alien/toe_alien_fleet_carrier.md](./units/toe/naval/alien/toe_alien_fleet_carrier.md)**：エイリアン航空艦隊 詳細編成・生態仕様書
  * 浮遊母艦生体（バイオキャリア）、艦載小型飛行種、警戒生体による洋上生体航空拠点群。
* **[units/toe/naval/alien/toe_alien_fleet_submarine.md](./units/toe/naval/alien/toe_alien_fleet_submarine.md)**：エイリアン潜水艦隊 詳細編成・生態仕様書
  * 深海潜航巨獣、超音波共鳴種、誘導生体魚雷放出種による完全隠密・深海奇襲雷撃群。

#### 多種族混成海上戦闘団および通常艦隊上位世代（Gen 4）

* **[units/toe/naval/multi_species/toe_naval_multi_species.md](./units/toe/naval/multi_species/toe_naval_multi_species.md)**：多種族混成海上戦闘団 詳細編成仕様書
  * 人類ミサイル軍艦と同盟海棲戦闘怪獣の混成運用（生体ソナー・魚雷迎撃）。
* **[units/toe/naval/fusion/toe_naval_fusion.md](./units/toe/naval/fusion/toe_naval_fusion.md)**：通常海上戦闘団・上位世代発展仕様書（生体技術・耐圧甲殻改修 / Gen 4）
  * 通常艦艇の近代化改修。有機装甲、無音鰭推進、生体魚雷射出管、潜航・浮上両用化仕様。

### 航空戦闘団（`units/toe/air/`）

#### 通常航空戦闘団（人類通常航空隊）（`units/toe/air/regular/`）

* **[units/toe/air/regular/toe_air_wing_civilian.md](./units/toe/air/regular/toe_air_wing_civilian.md)**：民間航空団 詳細編成・運用仕様書
  * 大型旅客機16機、コンテナ貨物機16〜32機による惑星内高速旅客・戦略物資定期空輸部隊。
* **[units/toe/air/regular/toe_air_wing_armed_civilian.md](./units/toe/air/regular/toe_air_wing_armed_civilian.md)**：武装民間航空団 詳細編成・運用仕様書
  * 機銃ポッド・自衛火器を仮設した武装貨物機32機による開拓初期・急造自衛航空部隊。
* **[units/toe/air/regular/toe_air_wing_tactical_transport.md](./units/toe/air/regular/toe_air_wing_tactical_transport.md)**：輸送航空団 詳細編成・運用仕様書
  * C-17級重戦術軍用輸送機32機による陸上部隊・装甲車両の不整地空輸・戦術投下部隊。
* **[units/toe/air/regular/toe_air_wing_patrol_aew.md](./units/toe/air/regular/toe_air_wing_patrol_aew.md)**：哨戒航空団 詳細編成・運用仕様書
  * 早期警戒機（AEW&C 8機）、対潜哨戒機16機、護衛機16機による広域索敵・早期警戒・対潜監視部隊。
* **[units/toe/air/regular/toe_air_wing_escort.md](./units/toe/air/regular/toe_air_wing_escort.md)**：護衛航空団 詳細編成・運用仕様書
  * 長航続距離大型制空戦闘機32機による輸送機・爆撃機・民間船団航路の専従直衛部隊。
* **[units/toe/air/regular/toe_air_wing_interceptor.md](./units/toe/air/regular/toe_air_wing_interceptor.md)**：迎撃航空団 詳細編成・運用仕様書
  * 軽戦闘機32〜48機による前線拠点・高速道路からの緊急スクランブル即応迎撃部隊。
* **[units/toe/air/regular/toe_air_wing_vstol.md](./units/toe/air/regular/toe_air_wing_vstol.md)**：垂直離陸航空団 詳細編成・運用仕様書
  * SVTOL戦闘機32機による無滑走路前線拠点・空母での即応垂直展開部隊。
* **[units/toe/air/regular/toe_air_wing_tactical_fighter.md](./units/toe/air/regular/toe_air_wing_tactical_fighter.md)**：戦闘航空団 詳細編成・運用仕様書
  * 大型制空戦闘機32機＋戦闘爆撃機16機による制空権確保および対地精密打撃の中核主力航空隊。
* **[units/toe/air/regular/toe_air_wing_stealth_fighter.md](./units/toe/air/regular/toe_air_wing_stealth_fighter.md)**：光学・電波ステルス戦闘航空団 詳細編成・運用仕様書
  * 光学迷彩・電波吸収ステルス戦闘機32機による完全不可視奇襲部隊。
* **[units/toe/air/regular/toe_air_wing_bomber.md](./units/toe/air/regular/toe_air_wing_bomber.md)**：爆撃航空団 詳細編成・運用仕様書
  * 大型戦略爆撃機16機＋護衛戦闘機16機による戦略絨毯爆撃・面制圧部隊。
* **[units/toe/air/regular/toe_air_wing_stealth_bomber.md](./units/toe/air/regular/toe_air_wing_stealth_bomber.md)**：光学・電波ステルス爆撃航空団 詳細編成・運用仕様書
  * 光学ステルス爆撃機8機＋護衛機16機による防空火網突破・中枢精密爆撃部隊。
* **[units/toe/air/regular/toe_air_wing_aerial_battleship.md](./units/toe/air/regular/toe_air_wing_aerial_battleship.md)**：空中戦艦航空団 詳細編成・運用仕様書
  * 超大型空中戦艦（1〜2隻）＋空中駆逐艦4隻＋空中フリゲート8隻＋直衛戦闘機32機による空中接近制圧・艦隊決戦部隊。
* **[units/toe/air/regular/toe_air_wing_aerial_carrier.md](./units/toe/air/regular/toe_air_wing_aerial_carrier.md)**：空中空母航空団 詳細編成・運用仕様書
  * 超大型空中空母（1隻）＋艦載戦闘機48機＋空中護衛艦艇による移動式空中航空基地部隊。
* **[units/toe/air/regular/toe_air_wing_stealth_recon.md](./units/toe/air/regular/toe_air_wing_stealth_recon.md)**：光学・電波ステルス偵察航空団 詳細編成・運用仕様書
  * 超音速光学ステルス偵察機16機による超高高度深部領空侵入・全域精密マッピング部隊。
* **[units/toe/air/regular/toe_air_wing_super_heavy_transport.md](./units/toe/air/regular/toe_air_wing_super_heavy_transport.md)**：超大型輸送航空団 詳細編成・運用仕様書
  * 超巨大戦略輸送機/飛行船4隻＋空中護衛艦艇による軍団・艦隊単位の超長距離空輸部隊（戦艦艦隊を凌駕するコスト）。

#### 野生・固有エイリアン航空群（エイリアン航空団）（`units/toe/air/alien/`）

* **[units/toe/air/alien/toe_alien_air_wing_transport.md](./units/toe/air/alien/toe_alien_air_wing_transport.md)**：エイリアン輸送航空団 詳細編成・生態仕様書
  * 巨大嚢胞飛行獣、生体空輸コンテナ種、護衛飛行種による地上生体・物資空輸降下群。
* **[units/toe/air/alien/toe_alien_air_wing_patrol.md](./units/toe/air/alien/toe_alien_air_wing_patrol.md)**：エイリアン哨戒航空団 詳細編成・生態仕様書
  * 大気浮遊巨大クラゲ（生体レーダー個体）、長距離滑空索敵種による高高度早期警戒・索敵群。
* **[units/toe/air/alien/toe_alien_air_wing_interceptor.md](./units/toe/air/alien/toe_alien_air_wing_interceptor.md)**：エイリアン迎撃航空団 詳細編成・生態仕様書
  * 小型高速有翼爪獣、生体短距離迎撃針射出種による即時緊急スクランブル迎撃群。
* **[units/toe/air/alien/toe_alien_air_wing_fighter.md](./units/toe/air/alien/toe_alien_air_wing_fighter.md)**：エイリアン戦闘航空団 詳細編成・生態仕様書
  * 中型重戦闘ワイバーン、強酸弾投射種、格闘鉤爪個体による制空権掌握・急降下襲撃主力群。
* **[units/toe/air/alien/toe_alien_air_wing_stealth_fighter.md](./units/toe/air/alien/toe_alien_air_wing_stealth_fighter.md)**：エイリアンステルス戦闘航空団 詳細編成・生態仕様書
  * 光学擬態有翼種、低赤外線放出捕食個体による不可視奇襲戦闘群。
* **[units/toe/air/alien/toe_alien_air_wing_bomber.md](./units/toe/air/alien/toe_alien_air_wing_bomber.md)**：エイリアン爆撃航空団 詳細編成・生態仕様書
  * 重爆撃巨大飛行獣、有機クラスター酸爆弾投射種による地上目標面制圧・戦略破壊群。
* **[units/toe/air/alien/toe_alien_air_wing_aerial_battleship.md](./units/toe/air/alien/toe_alien_air_wing_aerial_battleship.md)**：エイリアン空中戦艦航空団 詳細編成・生態仕様書
  * 超大型空中生体戦艦（リヴァイアサン飛行種）、直協飛行護衛群による空中決戦・重プラズマ砲制圧要塞生物。
* **[units/toe/air/alien/toe_alien_air_wing_aerial_carrier.md](./units/toe/air/alien/toe_alien_air_wing_aerial_carrier.md)**：エイリアン空中空母航空団 詳細編成・生態仕様書
  * 超巨大気嚢浮遊母艦獣、艦載小型突撃飛行種による移動空中巣穴・飛行生体産生母体。
* **[units/toe/air/alien/toe_alien_air_wing_stealth_recon.md](./units/toe/air/alien/toe_alien_air_wing_stealth_recon.md)**：エイリアンステルス偵察航空団 詳細編成・生態仕様書
  * 高度擬態長距離飛行種、生体電波吸収膜保有個体による深部不可視侵入・索敵群。
* **[units/toe/air/alien/toe_alien_air_wing_super_heavy_transport.md](./units/toe/air/alien/toe_alien_air_wing_super_heavy_transport.md)**：エイリアン超大型輸送航空団 詳細編成・生態仕様書
  * 巨大型嚢胞輸送怪獣、浮遊護衛飛行群によるタイタン級決戦生体の一括戦域間空輸群。

#### 多種族混成航空戦闘団および通常航空隊上位世代（Gen 4）

* **[units/toe/air/multi_species/toe_air_multi_species.md](./units/toe/air/multi_species/toe_air_multi_species.md)**：多種族混成航空戦闘団 詳細編成仕様書
  * パイロット搭乗型サイバネ飛行獣、胞子散布バイオファイターによる極限格闘戦。
* **[units/toe/air/fusion/toe_air_fusion.md](./units/toe/air/fusion/toe_air_fusion.md)**：通常航空戦闘団・上位世代発展仕様書（生体技術・可変有機翼改修 / Gen 4）
  * 通常航空隊の近代化改修。有機可変翼ジェット機、生体プラズマ爆撃機、無着陸長期滞空バイオアビオニクス機。

---

## 🔧 4. 兵器・装備カタログ仕様書（`units/equipment/`）

陸・海・空の各作戦領域における主要兵器プラットフォーム、共通車体・船体・機体ファミリー、および搭載火器・耐用年数（消耗度）カタログ。

### 陸上装備カタログ（`units/equipment/ground/`）

* **[units/equipment/ground/regular/ground_regular_equipment.md](./units/equipment/ground/regular/ground_regular_equipment.md)**：陸上・正規軍 兵器・車両プラットフォーム仕様書
  * 重装軌戦車系列、装軌装甲車系列（分隊固定8名）、8輪装甲装輪車系列（10大バリアント共通化率95%）、重装輪トラック系列（160C装輪自走榴弾砲）、4×4軽装甲車系列、二脚歩行戦闘車両。
* **[units/equipment/ground/alien/ground_alien_equipment.md](./units/equipment/ground/alien/ground_alien_equipment.md)**：陸上・エイリアン 生体兵器・個体仕様書
  * 小型スウォーム種、中型格闘種、大型戦車型、超大型タイタン級（コロッサス）、長距離ニードル砲撃種、陸棲航空ワイバーン種。

### 海上装備カタログ（`units/equipment/naval/`）

* **[units/equipment/naval/regular/naval_regular_equipment.md](./units/equipment/naval/regular/naval_regular_equipment.md)**：海上・正規軍 艦艇・装備プラットフォーム仕様書
  * 超弩級水上決戦艦（レールガン主砲）、航空母艦、外洋主力駆逐艦、外洋哨戒・対潜フリゲート、高速近接護衛コルベット、特設・民間輸送船艇。
* **[units/equipment/naval/alien/naval_alien_equipment.md](./units/equipment/naval/alien/naval_alien_equipment.md)**：海上・エイリアン 生体艦艇・海洋種仕様書
  * 小型刺胞遊泳個体、高速魚雷獣、棘皮防空個体、中型重装甲甲殻獣、超大型リヴァイアサン級決戦巨獣、バイオキャリア、深海潜航巨獣。

### 航空装備カタログ（`units/equipment/air/`）

* **[units/equipment/air/regular/air_regular_equipment.md](./units/equipment/air/regular/air_regular_equipment.md)**：航空・正規軍 航空機・装備プラットフォーム仕様書
  * 戦術戦闘機、迎撃戦闘機、ステルス戦闘機・爆撃機、戦略重爆撃機、垂直離着陸（SVTOL）機、戦術・戦略輸送機、超大型空中戦艦・空中空母。
* **[units/equipment/air/alien/air_alien_equipment.md](./units/equipment/air/alien/air_alien_equipment.md)**：航空・エイリアン 飛行生体・航空個体仕様書
  * 小型高速有翼爪獣、中型重戦闘ワイバーン、光学擬態ステルス飛行種、重爆撃巨大生体、超大型空中生体戦艦、気嚢浮遊母艦獣。

---

## 🛠️ 5. 技術設計・アーキテクチャ（`tech/`）

* **[tech/SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md](./tech/SYSTEM_ARCHITECTURE_AND_DEPENDENCIES.md)**：全体実装依存関係・アーキテクチャ仕様書
  * システム全体の6層アーキテクチャ（エンジン・空間・経済・軍事・動態・UI）、モジュール間依存関係マトリクス、およびボトムアップ実装ロードマップ。
* **[tech/BEVY_0_20_INTEGRATION_REVIEW.md](./tech/BEVY_0_20_INTEGRATION_REVIEW.md)**：Bevy 0.20 仕様照合・技術検討書
  * Bevy 0.20 新機能（Weak System Ordering、`Val::Em`/`Rem`、`FixedNode`、WESL、標準カメラ等）と現行実装の適合状況、およびUIレスポンシブ化・並列化等の拡張ロードマップ。
* **[tech/RESOURCE_IMPLEMENTATION_BRIDGE.md](./tech/RESOURCE_IMPLEMENTATION_BRIDGE.md)**：資源系 実装橋渡し案
  * 資源経済設計とBevy実装コードを繋ぐ暫定ブリッジ仕様（データ構造、タイル収支、段階的実装フェーズ計画）。
* **[tech/STARTUP_OPTIONS.md](./tech/STARTUP_OPTIONS.md)**：起動オプション・コマンドライン引数仕様書
  * コマンドライン引数（`--state` / `-s`）による初期画面（インゲーム、勢力選択、設定画面等）への直行機能、開発・テスト用ショートカット、デバッグコンソール連携および将来的な拡張CLIオプション（シード値固定・解像度・ヘッドレス等）構想。

---

## 📋 6. 開発計画・ロードマップ・課題（`planning/`）

* **[planning/CONSISTENCY_AUDIT_PLAN.md](./planning/CONSISTENCY_AUDIT_PLAN.md)**：仕様・ドキュメント整合性チェック計画書
  * ユーザー策定コア文書をマスターとし、周辺・全体文書へ段階的に逆還元（バックフィード）する実行計画と進捗管理。
* **[planning/MISSING_DOCUMENTS_ROADMAP.md](./planning/MISSING_DOCUMENTS_ROADMAP.md)**：未作成・不足ドキュメント体系と開発ロードマップ
  * 全体ドキュメントの最新成熟度マトリクス、明確に不足している5大領域（資源品目定義、技術ツリー、外交危機、エイリアン生態巣窟、ステアリングUI）、および開発ロードマップ。
* **[planning/PENDING_ISSUES.md](./planning/PENDING_ISSUES.md)**：未確定・検討課題まとめ（論点整理）
  * 歩行兵器の定義と不便さの具現化、国家概念の肉付け、戦闘団×モジュール共通化の接続など、掘り下げ不足な論点と今後の検討課題一覧。
* **[planning/SECONDARY_MISSING_SPECS.md](./planning/SECONDARY_MISSING_SPECS.md)**：二次的不足ドキュメント・システム統合仕様書体系
  * 概念・世界観から実装・結合ロジックへの橋渡しに必要な二次的仕様書（兵站状態遷移、UIフロー、データ結合）の整理。
* **[planning/SECONDARY_SPECS_EVALUATION.md](./planning/SECONDARY_SPECS_EVALUATION.md)**：2次的不足仕様・検討論点と実装アプローチ評価書
  * 二次的仕様書の各論点（兵站・UI・データ構造）に関する実装アプローチの比較・評価。
* **[planning/NEXT_PLAN_LAND_BATTLESHIP_AND_AIR_NAVAL_REORG.md](./planning/NEXT_PLAN_LAND_BATTLESHIP_AND_AIR_NAVAL_REORG.md)**：次期設計計画：陸上戦艦・艦隊航空隊整理
  * 航海可能な陸上戦艦、通常艦隊・航空隊の規模エシュロン再編、空棲・水棲エイリアン整理の覚書。

---

## 💡 7. 詳細検討・発展仕様（`discussion/`）

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
* **[concept_advanced_tech_dew_unmanned_and_naval_doctrine.md](./discussion/military_tactics/concept_advanced_tech_dew_unmanned_and_naval_doctrine.md)**
  * **先端軍事テクノロジー・水上運用ドクトリン長期総合検討書**。
  * 空・陸・海・潜の無人自律システム（UxV: UAV/UGV/USV/UUV）の4階層展開と有人指揮ノード（MUM-T）、エッジAI交戦・電子妨害耐性と倫理キルチェーン。
  * 高出力レーザー（HEL: 光速・弾薬無制限と大気・瘴気減衰）、高出力マイクロ波（HPM: ドローンスウォーム一括無力化・エイリアン神経撹乱）、VRM中間エネルギーバッファと排熱工学。
  * 水上システム総合運用ドクトリン（臨海造船所建造、陸上戦闘艦の海上自航回航・護衛、海岸揚陸、メガフロート/自走MOB、アイアンボトム・サウンドの海底サルベージ）。
  * 技術世代（Gen 0〜4）とアフィニティ進化、長期実装ロードマップ。

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
  * 巣窟の5大フェーズ（休眠/活性/繁殖/暴走スウォーム/枯渇）、瘴気の動的変容と流体拡散・浄化・環境適応。
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
