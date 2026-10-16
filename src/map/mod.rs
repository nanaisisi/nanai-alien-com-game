use bevy::prelude::*;

use crate::state::AppState;

pub mod generation;
pub mod hex;
pub mod input;
pub mod interaction;
pub mod settings;
pub mod terrain;
pub mod types;

pub use self::generation::{cleanup_hex_map, create_hex_mesh, generate_hex_map};
pub use self::hex::{MAP_HEIGHT as GRID_HEIGHT, MAP_WIDTH as GRID_WIDTH};
pub use self::settings::MapConfig;
pub use self::types::{HEX_RADIUS, MapDisplaySettings, MapGrid};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapConfig>()
            .init_resource::<MapGrid>()
            .init_resource::<MapDisplaySettings>()
            .add_plugins((input::MapInputPlugin, interaction::MapInteractionPlugin))
            .add_systems(OnEnter(AppState::InGame), generate_hex_map)
            .add_systems(OnEnter(AppState::Title), cleanup_hex_map);
    }
}
