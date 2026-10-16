use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod types;
pub mod view;

pub use interaction::{
    button_action_system, button_interaction_system, title_keyboard_navigation_system,
    update_title_focus_highlight_system,
};
pub use types::{TitleMenuFocus, reset_title_focus};
pub use view::{cleanup_title_ui, setup_title_ui};

pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleMenuFocus>()
            .add_systems(
                OnEnter(AppState::Title),
                (setup_title_ui, reset_title_focus),
            )
            .add_systems(
                Update,
                (
                    title_keyboard_navigation_system,
                    button_interaction_system,
                    button_action_system,
                    update_title_focus_highlight_system,
                )
                    .chain_weak()
                    .run_if(in_state(AppState::Title)),
            )
            .add_systems(OnExit(AppState::Title), cleanup_title_ui);
    }
}
