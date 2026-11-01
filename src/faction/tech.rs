use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// 3大アフィニティ（適応思想）の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AffinityType {
    /// Purity（純血・母星化）: 重化学工業、テラフォーミング、大口径火砲
    Purity,
    /// Harmony（調和・共生）: 異星生態系共生、生体触媒、キチン装甲
    Harmony,
    /// Supremacy（超越・自律）: 完全無人化、量子AI、指向性エネルギー
    Supremacy,
}

/// 3大アフィニティ蓄積ポイント
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub struct AffinityScore {
    pub purity: u32,
    pub harmony: u32,
    pub supremacy: u32,
}

impl AffinityScore {
    pub fn new(purity: u32, harmony: u32, supremacy: u32) -> Self {
        Self {
            purity,
            harmony,
            supremacy,
        }
    }

    /// 累積ポイントに応じたレベル（Lv 0 〜 10）を算出
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

    pub fn add(&mut self, other: &AffinityScore) {
        self.purity += other.purity;
        self.harmony += other.harmony;
        self.supremacy += other.supremacy;
    }
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

impl TechWebRegistry {
    /// CSVテキストから技術ノード群をロード
    pub fn load_from_csv(csv_content: &str) -> Result<Self, String> {
        let mut nodes = HashMap::new();
        let lines: Vec<&str> = csv_content.lines().collect();
        if lines.is_empty() {
            return Ok(Self { nodes });
        }

        for (idx, line) in lines.iter().enumerate().skip(1) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // CSV行のパース（クォート対応）
            let fields = Self::parse_csv_line(line);
            if fields.len() < 11 {
                return Err(format!(
                    "Invalid CSV line {}: expected 11 fields, got {}",
                    idx + 1,
                    fields.len()
                ));
            }

            let id = fields[0].clone();
            let name_ja = fields[1].clone();
            let name_en = fields[2].clone();
            let tier: u8 = fields[3]
                .parse()
                .map_err(|e| format!("Line {}: invalid tier: {}", idx + 1, e))?;
            let is_trunk = fields[4].to_lowercase() == "true";
            let prerequisites = if fields[5].is_empty() {
                Vec::new()
            } else {
                fields[5].split('|').map(|s| s.trim().to_string()).collect()
            };
            let required_rp: u32 = fields[6]
                .parse()
                .map_err(|e| format!("Line {}: invalid required_rp: {}", idx + 1, e))?;
            let purity: u32 = fields[7]
                .parse()
                .map_err(|e| format!("Line {}: invalid purity: {}", idx + 1, e))?;
            let harmony: u32 = fields[8]
                .parse()
                .map_err(|e| format!("Line {}: invalid harmony: {}", idx + 1, e))?;
            let supremacy: u32 = fields[9]
                .parse()
                .map_err(|e| format!("Line {}: invalid supremacy: {}", idx + 1, e))?;
            let unlocks = if fields[10].is_empty() {
                Vec::new()
            } else {
                fields[10].split(';').map(|s| s.trim().to_string()).collect()
            };

            nodes.insert(
                id.clone(),
                TechNodeDefinition {
                    id,
                    name_ja,
                    name_en,
                    tier,
                    is_trunk,
                    prerequisites,
                    required_rp,
                    affinity_gain: AffinityScore::new(purity, harmony, supremacy),
                    unlocks,
                },
            );
        }

        Ok(Self { nodes })
    }

    /// 前提条件DAGの循環参照・未定義ノードのチェック
    pub fn validate_dag(&self) -> Result<(), String> {
        for (id, node) in &self.nodes {
            for prereq in &node.prerequisites {
                if !self.nodes.contains_key(prereq) {
                    return Err(format!(
                        "Node {} references unknown prerequisite {}",
                        id, prereq
                    ));
                }
            }
        }

        // 循環チェック (DFS)
        let mut visited = HashMap::new(); // 0: unvisited, 1: visiting, 2: visited
        for id in self.nodes.keys() {
            visited.insert(id.as_str(), 0);
        }

        fn dfs<'a>(
            node_id: &'a str,
            visited: &mut HashMap<&'a str, u8>,
            nodes: &'a HashMap<String, TechNodeDefinition>,
        ) -> Result<(), String> {
            visited.insert(node_id, 1);
            if let Some(node) = nodes.get(node_id) {
                for prereq in &node.prerequisites {
                    let p_str = prereq.as_str();
                    match visited.get(p_str) {
                        Some(1) => {
                            return Err(format!(
                                "Cycle detected involving node {} and {}",
                                node_id, prereq
                            ))
                        }
                        Some(0) => dfs(p_str, visited, nodes)?,
                        _ => {}
                    }
                }
            }
            visited.insert(node_id, 2);
            Ok(())
        }

        for id in self.nodes.keys() {
            if visited.get(id.as_str()) == Some(&0) {
                dfs(id.as_str(), &mut visited, &self.nodes)?;
            }
        }

        Ok(())
    }

    fn parse_csv_line(line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut cur = String::new();
        let mut in_quotes = false;

        for ch in line.chars() {
            if ch == '"' {
                in_quotes = !in_quotes;
            } else if ch == ',' && !in_quotes {
                fields.push(cur.trim().to_string());
                cur.clear();
            } else {
                cur.push(ch);
            }
        }
        fields.push(cur.trim().to_string());
        fields
    }
}

/// 勢力の研究開発進捗コンポーネント
#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct FactionResearchProgress {
    /// 解禁済み技術ID一覧
    pub unlocked_techs: HashSet<String>,
    /// 各技術の蓄積研究ポイント (RP)
    pub accumulated_rp: HashMap<String, u32>,
    /// 現場起因の自律研究モメンタム (毎ターン蓄積される自然進捗)
    pub research_momentum: HashMap<String, f32>,
    /// 指導者が重点指定したステアリング対象技術ID
    pub steering_target: Option<String>,
    /// ステアリング移行ラグ（停滞ターン）
    pub steering_friction_turns: u8,
}

impl FactionResearchProgress {
    /// 降着基盤等の初期技術を解禁した状態で生成
    pub fn new_with_initial(initial_techs: &[&str]) -> Self {
        let mut unlocked = HashSet::new();
        for t in initial_techs {
            unlocked.insert(t.to_string());
        }
        Self {
            unlocked_techs: unlocked,
            accumulated_rp: HashMap::new(),
            research_momentum: HashMap::new(),
            steering_target: None,
            steering_friction_turns: 0,
        }
    }

    /// 毎ターンのRP進捗更新および技術完了・アフィニティ加算処理
    pub fn advance_turn(
        &mut self,
        national_rp: u32,
        registry: &TechWebRegistry,
        affinity: &mut AffinityScore,
    ) -> Vec<String> {
        // 1. 現場モメンタムの自律注入
        for (tech_id, momentum) in &self.research_momentum {
            if self.is_researchable(tech_id, registry) {
                let entry = self.accumulated_rp.entry(tech_id.clone()).or_insert(0);
                *entry += *momentum as u32;
            }
        }

        // 2. 国家ステアリングRPの集中注入
        if let Some(target_id) = &self.steering_target {
            if self.is_researchable(target_id, registry) {
                let inject_rp = if self.steering_friction_turns > 0 {
                    self.steering_friction_turns -= 1;
                    national_rp / 2 // フリクション減衰 (50%)
                } else {
                    national_rp
                };
                let entry = self.accumulated_rp.entry(target_id.clone()).or_insert(0);
                *entry += inject_rp;
            }
        }

        // 3. 技術完了判定
        let mut newly_unlocked = Vec::new();
        for (tech_id, rp) in &self.accumulated_rp {
            if let Some(node) = registry.nodes.get(tech_id) {
                if *rp >= node.required_rp && !self.unlocked_techs.contains(tech_id) {
                    newly_unlocked.push(tech_id.clone());
                }
            }
        }

        // 4. 解禁適用とアフィニティ加算
        for tech_id in &newly_unlocked {
            self.unlocked_techs.insert(tech_id.clone());
            if let Some(node) = registry.nodes.get(tech_id) {
                affinity.add(&node.affinity_gain);
            }
        }

        newly_unlocked
    }

    /// 当該技術が研究可能（未解禁かつ前提条件をすべて満たしているか）
    pub fn is_researchable(&self, tech_id: &str, registry: &TechWebRegistry) -> bool {
        if self.unlocked_techs.contains(tech_id) {
            return false;
        }
        if let Some(node) = registry.nodes.get(tech_id) {
            node.prerequisites
                .iter()
                .all(|p| self.unlocked_techs.contains(p))
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CSV: &str = include_str!("../../assets/data/tech_nodes.csv");

    #[test]
    fn test_csv_loading_and_dag_validation() {
        let registry = TechWebRegistry::load_from_csv(TEST_CSV).expect("Failed to load CSV");
        assert_eq!(registry.nodes.len(), 36, "Must contain all 36 nodes");
        assert!(registry.nodes.contains_key("TECH_CORE"));
        assert!(registry.nodes.contains_key("TECH_TRK_P3"));
        assert!(registry.nodes.contains_key("TECH_LF_S32"));

        // DAG整合性の検証（循環なし、未定義ノードなし）
        assert!(registry.validate_dag().is_ok());
    }

    #[test]
    fn test_affinity_level_progression() {
        let mut affinity = AffinityScore::default();
        assert_eq!(affinity.level_of(AffinityType::Purity), 0);

        affinity.purity = 2;
        assert_eq!(affinity.level_of(AffinityType::Purity), 1);

        affinity.purity = 9;
        assert_eq!(affinity.level_of(AffinityType::Purity), 3);

        affinity.purity = 27;
        assert_eq!(affinity.level_of(AffinityType::Purity), 6);

        affinity.purity = 65;
        assert_eq!(affinity.level_of(AffinityType::Purity), 10);
    }

    #[test]
    fn test_research_progress_and_unlock() {
        let registry = TechWebRegistry::load_from_csv(TEST_CSV).expect("Failed to load CSV");
        let mut progress = FactionResearchProgress::new_with_initial(&["TECH_CORE"]);
        let mut affinity = AffinityScore::default();

        assert!(!progress.is_researchable("TECH_CORE", &registry));
        assert!(progress.is_researchable("TECH_C_01", &registry));

        // TECH_C_01 をステアリング対象に設定 (所要50 RP, Purity +1)
        progress.steering_target = Some("TECH_C_01".to_string());
        progress.steering_friction_turns = 1;

        // Turn 1: 50 RP供給 -> フリクションで25 RP注入 (未完了)
        let unlocked = progress.advance_turn(50, &registry, &mut affinity);
        assert!(unlocked.is_empty());
        assert_eq!(progress.accumulated_rp.get("TECH_C_01"), Some(&25));
        assert_eq!(affinity.purity, 0);

        // Turn 2: 50 RP供給 -> 100%効率で50 RP注入 -> 累計75 RP >= 50 RP で完了
        let unlocked = progress.advance_turn(50, &registry, &mut affinity);
        assert_eq!(unlocked, vec!["TECH_C_01".to_string()]);
        assert!(progress.unlocked_techs.contains("TECH_C_01"));
        assert_eq!(affinity.purity, 1);

        // TECH_C_01解禁に伴い TECH_TRK_P1 が研究可能になる
        assert!(progress.is_researchable("TECH_TRK_P1", &registry));
    }
}
