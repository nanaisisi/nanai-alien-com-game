use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod mesh;
pub mod spawn;
pub mod types;

pub use interaction::UnitInteractionPlugin;
pub use spawn::{cleanup_units, spawn_initial_units};
pub use types::{
    CombatGroupType, DeploymentStatus, HangarBay, MoveModeState, NationalArsenalPool, SelectedUnit,
    Unit, UnitActionState,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NationalArsenalPool>()
            .register_type::<Unit>()
            .register_type::<CombatGroupType>()
            .register_type::<UnitActionState>()
            .register_type::<DeploymentStatus>()
            .register_type::<HangarBay>()
            .add_plugins(UnitInteractionPlugin)
            .add_systems(
                OnEnter(AppState::InGame),
                spawn_initial_units.after(crate::faction::setup_initial_faction_territories),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_units);
    }
}
