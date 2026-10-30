# SEC-11: 外交エスカレーション・和平条約・属国化交渉仕様書

本仕様書は、勢力間（架空6大国家など）の外交的緊張から武力衝突、そして終戦から戦後処理（講和・属国化）に至る一連の外交システムと計算ロジックを規定する。

---

## 1. 緊張度推移とエスカレーション・ラダー

平和状態からいきなり全面戦争が始まるのではなく、グレーゾーン事態を含む多段階のエスカレーションを経る。

### 1.1 緊張度・摩擦インデックス（Tension & Friction Index）

2国間の関係性は「Tension（緊張度）」パラメータ（0〜100）で管理される。

- **緊張上昇要因**:
  - 国境未定地（グレーゾーン）への軍事ユニット・開拓団の侵入。
  - スパイ活動の発覚。
  - 他国の同盟国への攻撃。
  - 思想（アフィニティ）の著しい対立（例：Purity国家がHarmony国家の「エイリアン保護」を非難）。
- **摩擦（Friction）イベント**: 緊張度が高いと、「民間船の拿捕」「国境での偶発的発砲」などのランダムな摩擦イベントが発生し、プレイヤーに対応（謝罪、強硬姿勢）が求められる。

### 1.2 4段階エスカレーション・ラダー

緊張度に応じて、以下の外交フェーズへ移行する。

1.  **Level 0: Peace（平和・通商）**: 自由な交易、国境開放条約の締結が可能。
2.  **Level 1: Diplomatic Warning（警告・非難）**: 緊張度30〜。交易効率低下。特定技術の輸出禁止措置。
3.  **Level 2: Economic Sanctions（経済制裁・資産凍結）**: 緊張度60〜。国境閉鎖。交易完全停止。戦略資源（エネルギー・鉱物）の禁輸。
4.  **Level 3: Border Conflict（局地紛争）**: 緊張度80〜。最後通牒（Ultimatum）。特定の国境タイルやグレーゾーンでのみ交戦が許可される（全面戦争ではない）。
5.  **Level 4: Total War（全面戦争）**: 緊張度100、または開戦事由（Casus Belli）の行使により移行。領土の占領・都市への攻撃が可能。

---

## 2. 講和・休戦交渉テーブル

戦争を終結させるための外交プロトコル。全面戦争状態からPeaceやLevel 1へ戻すためには条約が必要。

### 2.1 交渉材料（War Score & Demands）

戦闘での勝利、都市の占領により「戦勝点（War Score）」が蓄積され、これを消費して相手に要求（Demands）を呑ませる。

- **領土割譲（Cede Territory）**: 占領下にある、または自然国境（河川・山脈）に基づくタイルの正式な割譲。
- **非武装地帯（DMZ）設定**: 指定したタイルに軍事ユニット（民兵以外）を配置できないようにする。
- **賠償金（Reparations）**: 毎ターンのエネルギー・鉱物資源・工業生産力（一定割合）を数ターン〜数十ターンにわたり上納させる。
- **武装解除（Disarmament）**: 相手の特定技術（大量破壊兵器、核融合など）の使用・生産を禁止する。

---

## 3. 降伏・属国化・主権解体プロトコル

国家の継戦能力が完全に失われた（首都陥落、全領土占領、モラル崩壊）場合の戦後処理。

### 3.1 敗戦国の解体と「二重経済（Divided Economy）」

無条件降伏した場合、その国家は消滅するか、属国（Puppet State / Vassal）として再編される。

- **二重経済システム**: 属国化された領土の経済は、宗主国への上納（収奪）と、現地での治安維持（最低限の生存）に分割される。
- プレイヤーは占領地に対して「苛烈な収奪（短期利益大、反乱確率大）」か「穏健な統治（短期利益小、同化促進）」を選択する。

### 3.2 ゲリラ化と第三極（自発的失踪者）

主権が解体された際、残存する旧軍部隊や強硬派の民間人は、勝者の統治を受け入れず地下へ潜る。

- **反乱軍・残党勢力**: 占領下のタイルに「治安度（Security）」が設定される。治安度が低いと、定期的にレジスタンス（民兵、旧軍のゲリラ）が蜂起し、インフラや補給線を破壊する。
- **自発的失踪者（Frontier Dissidents）**: 勝者にも敗者にも属さない、非正規の第三極勢力としてマップ周辺部や山岳地帯にコミュニティを形成する。彼らとの取引や討伐も新たなゲーム展開となる。

---

## 4. ECS データ構造（Rust/Bevy）実装案

```rust
#[derive(Component)]
pub struct DiplomaticRelations {
    pub faction_id_a: Entity,
    pub faction_id_b: Entity,
    pub tension: f32, // 0.0 to 100.0
    pub escalation_level: EscalationLevel,
    pub active_treaties: Vec<Treaty>,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum EscalationLevel {
    Peace,
    Warning,
    Sanctions,
    Conflict,
    TotalWar,
}

pub struct Treaty {
    pub treaty_type: TreatyType,
    pub duration_turns: Option<u32>, // None = 永続
}

pub enum TreatyType {
    OpenBorders,
    TradeAgreement,
    Ceasefire,
    Vassalage,
    DMZ(Vec<Entity>), // 対象となるタイル群
}

// 占領地の状態
#[derive(Component)]
pub struct OccupationState {
    pub original_owner: Entity,
    pub current_occupier: Entity,
    pub security_level: f32,
    pub economy_policy: OccupationPolicy,
}

pub enum OccupationPolicy {
    Exploitative, // 収奪
    Balanced,     // 均衡
    Integration,  // 同化
}
```
