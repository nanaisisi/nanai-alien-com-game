use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod types;
pub mod view;

pub use interaction::{
    pause_button_action_system, pause_button_interaction_system, pause_keyboard_navigation_system,
    update_pause_focus_highlight_system,
};
pub use types::{PauseMenuFocus, reset_pause_focus};
pub use view::{cleanup_pause_menu_ui, setup_pause_menu_ui};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseMenuFocus>()
            .add_systems(
                OnEnter(AppState::PauseMenu),
                (setup_pause_menu_ui, reset_pause_focus),
            )
            .add_systems(
                Update,
                (
                    pause_keyboard_navigation_system,
                    pause_button_interaction_system,
                    pause_button_action_system,
                    update_pause_focus_highlight_system,
                )
                    .chain_weak()
                    .run_if(in_state(AppState::PauseMenu)),
            )
            .add_systems(OnExit(AppState::PauseMenu), cleanup_pause_menu_ui);
    }
}
