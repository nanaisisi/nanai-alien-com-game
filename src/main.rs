use bevy::prelude::*;

mod camera;
mod faction;
mod map;
mod state;
mod ui;
mod unit;
mod world;

use camera::CameraPlugin;
use faction::FactionPlugin;
use map::MapPlugin;
use state::AppState;
use ui::GameUiPlugin;
use unit::UnitPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
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
        }),)
        .init_state::<AppState>()
        // Custom Plugins (Initialization and Logic Modules)
        .add_plugins((
            CameraPlugin,
            WorldPlugin,
            MapPlugin,
            FactionPlugin,
            UnitPlugin,
            GameUiPlugin,
        ))
        // State Management: Sets the default state and registers core gameplay systems
        .init_state::<AppState>()
        .add_systems(Startup, setup_world_environment) // Runs initialization from src/world.rs
        .add_systems(Update, (game_turn_system.run_if(in_state(AppState::InGame)), CameraSystem::default).chain()) // Core game loop logic
        .run();
}

/// ターン進行システム: リソース消費、部隊移動、マップ状態更新のトリガー。
fn game_turn_system(mut next_state: ResMut<NextState<AppState>>) {
    // TODO: ここにターン経過時の全てのゲームロジック（リソース計算、移動判定、戦闘解決など）を記述する
    println!("--- Game Turn System Activated ---");
}
