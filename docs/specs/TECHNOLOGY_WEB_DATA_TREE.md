# SEC-09: 技術ウェブ・研究開発データツリー仕様書

本仕様書は、ゲーム内の研究開発（R&D）システムである「技術ウェブ（Tech Web）」のデータ構造、技術ノードの定義、アフィニティ（思想・適応路線）ポイントの付与、および現場自律研究と国家ステアリングのメカニクスについて規定する。

---

## 1. 放射状技術ウェブ構造（Radial Tech Web）

ツリーは直線的ではなく、中央の「降着基盤（Core / Tier 0）」から外縁へ広がる放射状（Web）構造をとる。

### 1.1 階層構造（Trunk & Leaf）

- **Trunk（幹技術）**: 時代を画する主要パラダイム。研究コストが高く、新しい時代の兵器や施設群の前提となる。
- **Leaf（葉技術）**: Trunkから派生する特化技術・改良版。研究コストは低く、特定ユニットへのバフや特殊モジュールのアンロック、特定アフィニティの大幅な上昇をもたらす。

### 1.2 技術ティア（Tier）

- **Tier 0 (Gen 0/1)**: 初期開拓、生存と基本産業の立ち上げ。
- **Tier 1 (Gen 2)**: 独立した軍備と重工業の確立。
- **Tier 2 (Gen 3)**: 母星水準テクノロジーの復元（ステルス、精密誘導）。
- **Tier 3 (Gen 4)**: 異星環境への超克・特化進化（生体サイバネティクス、超重装甲）。

---

## 2. 全技術ノード（サンプル抜粋）

（※ 約30〜40ノードを想定。以下は主要な概念実装の例）

| ノードID      | ノード名（Trunk/Leaf）           | 前提技術                   | 解禁要素（モジュール・施設等）       | アフィニティ付与         |
| :------------ | :------------------------------- | :------------------------- | :----------------------------------- | :----------------------- |
| `TECH_CORE`   | 降着基盤 (Trunk, Tier 0)         | なし                       | 探査バギー、基礎インフラ、10C小火器  | なし                     |
| `TECH_IND_1`  | 粗鋼重工業 (Trunk, Tier 1)       | `TECH_CORE`                | Gen 1 車台、粗製溶接装甲、120C自走迫 | Purity +1                |
| `TECH_BIO_1`  | 異星生態系解析 (Trunk, Tier 1)   | `TECH_CORE`                | 瘴気観測所、解毒パック               | Harmony +1               |
| `TECH_AI_1`   | 戦術データリンク (Trunk, Tier 1) | `TECH_CORE`                | 基礎指揮車、自律監視タワー           | Supremacy +1             |
| `TECH_WPN_2`  | 標準化火器 (Leaf, Tier 1)        | `TECH_IND_1`               | C規格弾薬工場、160C榴弾砲            | Purity +2                |
| `TECH_BIO_L1` | 共生農法 (Leaf, Tier 1)          | `TECH_BIO_1`               | 異星作物プラント、食料産出+20%       | Harmony +2               |
| `TECH_MAT_3`  | 複合装甲材 (Trunk, Tier 2)       | `TECH_IND_1`               | Gen 3 複合装甲、主力戦車シャシー     | Purity +3, Supremacy +1  |
| `TECH_XEN_3`  | キチン重工複合 (Leaf, Tier 2)    | `TECH_MAT_3`, `TECH_BIO_1` | 生体甲殻装甲モジュール、自己修復     | Harmony +3               |
| `TECH_AUT_4`  | 完全自律戦闘群 (Trunk, Tier 3)   | `TECH_AI_1`                | 無人重歩行機、AI司令部               | Supremacy +5             |
| `TECH_MUT_4`  | 生体サイボーグ化 (Leaf, Tier 3)  | `TECH_BIO_1`, `TECH_AI_1`  | 多種族混成ユニット、精神リンク       | Harmony +3, Supremacy +2 |

---

## 3. アフィニティ（Affinity）システム

技術研究を完了することで、国家のアフィニティレベルが上昇する。アフィニティは部隊のドクトリンやエンディング（勝利条件）に直結する。

### 3.1 3大アフィニティ

1.  **Purity（純潔・母星化）**: 地球人類の姿と環境を維持し、惑星を母星環境へテラフォーミングする路線。重装甲、大火力、高コストな兵器体系。
2.  **Harmony（調和・共生）**: エイリアン生態系を理解し、遺伝子改変により人間を惑星環境に適応させる路線。生体装甲、瘴気回復、多種族混成部隊。
3.  **Supremacy（覇権・超越）**: 機械化、サイバネティクス、AIにより人間の限界を超越する路線。自律兵器スウォーム、高効率エネルギー、レーザー・電磁砲。

### 3.2 アフィニティレベルのボーナス

各アフィニティが一定レベル（Lv3, Lv6, Lv10など）に達するごとに、国家全休へのパッシブボーナスや、特殊な部隊の編成が解禁される。

---

## 4. 自律研究ベクトルとステアリング

プレイヤーが直接「次はこの技術を研究する」と指示するだけでなく、国家・現場の状況による自律的な研究進行が存在する。

### 4.1 自律研究のモメンタム（Momentum）

- **交戦経験**: 実戦でミサイルを多用すれば、「精密誘導」関連の技術への自律的な研究ポイント（RP）注入が加速する。
- **環境刺激**: 瘴気ダメージを多く受けている部隊がいれば、「異星生態系」や「環境防護」技術のモメンタムが上昇する。

### 4.2 国家指導者によるステアリング

プレイヤーはUIを通じて「重点特許指定」や「特別奨励金（予算投入）」を行い、自律研究の方向性を強力に誘導（ステアリング）することができる。ただし、モメンタムに反する研究（例：エイリアンと交戦ばかりしているのに、唐突に共生研究を指定する）は、研究完了までの移行ラグ（ペナルティ）が増大する。

---

## 5. 敵残骸法医学（Battlefield Forensics）

戦闘終了時、撃破された敵ユニットやエイリアンの残骸から技術ポイントを抽出するシステム。

- **リバースエンジニアリング**: 自国より上位世代（Tier）の敵兵器を撃破し回収した場合、関連技術ツリーへの即時RPボーナスを獲得する。
- **生体サンプル**: 巨大エイリアン（タイタン級など）を撃破すると、Harmony系技術を大幅にブーストする「高純度生体サンプル」を獲得できる。

---

## 6. ECS データ構造（Rust/Bevy）実装案

```rust
#[derive(Resource)]
pub struct TechWeb {
    pub nodes: HashMap<String, TechNode>,
    pub current_research: Option<String>,
}

pub struct TechNode {
    pub id: String,
    pub name: String,
    pub tier: u8,
    pub is_trunk: bool,
    pub dependencies: Vec<String>,
    pub base_cost_rp: u32,
    pub affinity_rewards: AffinityPoints,
}

#[derive(Component, Default, Clone, Copy)]
pub struct AffinityPoints {
    pub purity: u32,
    pub harmony: u32,
    pub supremacy: u32,
}

// 国家ごとの研究状態とモメンタム
#[derive(Component)]
pub struct FactionResearch {
    pub unlocked_techs: HashSet<String>,
    pub accumulated_rp: HashMap<String, u32>,
    pub research_momentum: HashMap<String, f32>, // 現場からの自律研究ベクトル
    pub steering_target: Option<String>, // プレイヤーの指定
}
```
