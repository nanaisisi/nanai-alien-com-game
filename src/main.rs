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
        // (setup_world_environment は WorldPlugin、カメラ制御は CameraPlugin 内で登録済み)
        // .add_systems(Startup, setup_world_environment)
        // .add_systems(Update, (game_turn_system.run_if(in_state(AppState::InGame)), CameraSystem::default).chain())
        .run();
}

// /// ターン進行システム: リソース消費、部隊移動、マップ状態更新のトリガー。
// fn game_turn_system(mut next_state: ResMut<NextState<AppState>>, mut core_state: ResMut<GameCoreState>) {
//     // TODO: ここにターン経過時の全てのゲームロジック（リソース計算、移動判定、戦闘解決など）を記述する
//     println!("--- Game Turn System Activated ---");
//
//     // 1. リソース消費フェーズ (消耗品の自動消費)
//     for resource in [state::ResourceType::Energy, state::ResourceType::Biomass] {
//         if let Some(res) = core_state.game_state.resources.get_mut(&resource) {
//             *res *= 0.95; // 例: 毎ターン5%消費減衰（メンテナンスコスト）
//             println!("Consumed {} resources. New value: {:.2}", format!("{:?}", resource), res);
//         }
//     }
//
//     // 2. 部隊移動・戦闘解決フェーズ (マップやユニットの更新)
//     // ここでMapPlugin, UnitPlugin内のシステムが呼び出され、実際のシミュレーションロジックを実行する想定。
//     // 例: World::update_unit_positions().run_if(in_state(AppState::InGame));
//
//     // 3. ターン進行の完了とリセット（または次のフェーズへ）
//     core_state.game_state.turn_number += 1;
// }
