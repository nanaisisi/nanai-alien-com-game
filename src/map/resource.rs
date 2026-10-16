use bevy::prelude::*;

/// 内部資源の品目（設計: doc/RESOURCE_ECONOMY_SYSTEM.md）
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RawResource {
    Energy,
    StructuralOre,
    RareMinerals,
    Biomass,
}

/// 基本産出。タイル産出・収支で共通利用（科学は研究ポイントとして別枠）。
/// 内部値は f32、表示のみ整数丸め。
#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ResourceAmounts {
    pub energy: f32,
    /// 生産 = 構造鉱物の採掘量
    pub ore: f32,
    /// 食料 = バイオマス
    pub biomass: f32,
    pub science: f32,
}

impl ResourceAmounts {
    pub const fn new(biomass: f32, ore: f32, energy: f32, science: f32) -> Self {
        Self {
            energy,
            ore,
            biomass,
            science,
        }
    }

    /// 現行UIと同じ並びの (食料, 生産, エネルギー, 科学)
    #[allow(dead_code)]
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.biomass, self.ore, self.energy, self.science)
    }

    /// 表示用に四捨五入した (食料, 生産, エネルギー, 科学)
    pub fn rounded(&self) -> (i32, i32, i32, i32) {
        (
            self.biomass.round() as i32,
            self.ore.round() as i32,
            self.energy.round() as i32,
            self.science.round() as i32,
        )
    }
}

impl std::ops::Add for ResourceAmounts {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(
            self.biomass + o.biomass,
            self.ore + o.ore,
            self.energy + o.energy,
            self.science + o.science,
        )
    }
}

impl std::ops::AddAssign for ResourceAmounts {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}

/// 特殊資源（希少鉱物など）。該当タイルのみ保持
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct TileSpecialResource {
    pub kind: RawResource,
    pub amount: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_round() {
        let a = ResourceAmounts::new(1.4, 0.0, 2.5, 0.0) + ResourceAmounts::new(0.2, 1.0, 0.0, 0.0);
        assert_eq!(a.rounded(), (2, 1, 3, 0));
    }
}
