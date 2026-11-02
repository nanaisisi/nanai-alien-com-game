pub mod mesh;
pub mod spawn;
pub mod systems;
pub mod types;

use bevy::prelude::*;

pub use types::{AlienNest, MiasmaMap, NestLifecycle};

use crate::state::AppState;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MiasmaMap>()
            .register_type::<NestLifecycle>()
            .register_type::<AlienNest>()
            .add_systems(
                Update,
                (
                    spawn::spawn_alien_nests.run_if(in_state(AppState::InGame)),
                    systems::handle_alien_turn_update.run_if(in_state(AppState::InGame)),
                ),
            )
            .add_systems(OnEnter(AppState::Title), systems::cleanup_alien_ecosystem);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::faction::types::FactionId;
    use crate::map::hex::HexCoord;
    use crate::unit::types::{CombatGroupType, Unit};

    #[test]
    fn test_alien_nest_irritation_and_lifecycle_transitions() {
        let mut nest = AlienNest::new(1, HexCoord::new(0, 0));
        assert_eq!(nest.lifecycle, NestLifecycle::Dormant);

        // Active閾値(25)への到達
        nest.add_irritation(30.0);
        assert_eq!(nest.lifecycle, NestLifecycle::Active);
        assert_eq!(nest.irritation, 30.0);

        // Breeding閾値(60)への到達
        nest.add_irritation(35.0);
        assert_eq!(nest.lifecycle, NestLifecycle::Breeding);
        assert_eq!(nest.irritation, 65.0);

        // Swarming閾値(90)への到達
        nest.add_irritation(30.0);
        assert_eq!(nest.lifecycle, NestLifecycle::Swarming);
        assert_eq!(nest.irritation, 95.0);

        // 刺激沈静化によるフェーズ逆戻り
        nest.add_irritation(-50.0);
        assert_eq!(nest.lifecycle, NestLifecycle::Active);
        assert_eq!(nest.irritation, 45.0);
    }

    #[test]
    fn test_miasma_map_get_set_clamping() {
        let mut miasma = MiasmaMap::default();
        let coord = HexCoord::new(5, 5);

        assert_eq!(miasma.get(&coord), 0.0);

        miasma.set(coord, 0.5);
        assert_eq!(miasma.get(&coord), 0.5);

        // 最大値1.0でのクランプ
        miasma.add(coord, 0.8);
        assert_eq!(miasma.get(&coord), 1.0);

        // 0.001以下での自動クリーンアップ
        miasma.set(coord, 0.0005);
        assert_eq!(miasma.get(&coord), 0.0);
        assert!(!miasma.concentrations.contains_key(&coord));
    }

    #[test]
    fn test_miasma_hazard_damage_to_unit() {
        let mut unit = Unit::new(
            FactionId::Empire,
            CombatGroupType::LightInfantry,
            HexCoord::new(0, 0),
        );
        let initial_hp = unit.hp;

        // 高濃度瘴気(>=0.7)のシミュレート: 最大HPの10%ダメージ
        let high_miasma_dmg = (unit.max_hp as f32 * 0.10).max(5.0) as u32;
        unit.hp = unit.hp.saturating_sub(high_miasma_dmg);

        assert!(unit.hp < initial_hp);
        assert_eq!(unit.hp, initial_hp - high_miasma_dmg);
    }
}
