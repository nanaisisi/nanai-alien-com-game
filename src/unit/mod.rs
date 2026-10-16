use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod mesh;
pub mod spawn;
pub mod types;

pub use interaction::UnitInteractionPlugin;
pub use spawn::{cleanup_units, spawn_initial_units};
pub use types::{CombatGroupType, MoveModeState, SelectedUnit, Unit, UnitActionState};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .register_type::<CombatGroupType>()
            .register_type::<UnitActionState>()
            .add_plugins(UnitInteractionPlugin)
            .add_systems(
                OnEnter(AppState::InGame),
                spawn_initial_units.after(crate::faction::setup_initial_faction_territories),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_units);
    }
}
