use bevy::prelude::*;

mod camera;
mod faction;
mod map;
mod state;
pub mod turn;
mod ui;
mod unit;
mod world;

use camera::CameraPlugin;
use faction::FactionPlugin;
use map::MapPlugin;
use state::AppState;
use turn::TurnPlugin;
use ui::GameUiPlugin;
use unit::UnitPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Nanai Alien Com Game".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    filter: format!("{},icu_provider=error", bevy::log::DEFAULT_FILTER),
                    ..default()
                }),
        )
        .insert_state(AppState::from_args())
        // Custom Plugins (Initialization and Logic Modules)
        .add_plugins((
            CameraPlugin,
            WorldPlugin,
            MapPlugin,
            FactionPlugin,
            TurnPlugin,
            UnitPlugin,
            GameUiPlugin,
        ))
        .run();
}
