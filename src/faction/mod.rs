use bevy::prelude::*;

use crate::state::AppState;

pub mod city_mesh;
pub mod industry;
pub mod infrastructure;
pub mod stockpile;
pub mod tech;
pub mod territory;
pub mod types;
pub mod upkeep;

pub use industry::IndustryGrid;
pub use infrastructure::InfrastructureGrid;
pub use territory::{
    FactionOutpost, TerritoryMap, TileTerritory, cleanup_territories,
    setup_initial_faction_territories, update_territory_overlays,
};
pub use types::{DiplomaticRelation, FactionId, FactionManager, FactionResources, PlayerFaction};
pub use upkeep::TileOperatingGrid;

pub struct FactionPlugin;

impl Plugin for FactionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(stockpile::StockpilePlugin)
            .init_resource::<PlayerFaction>()
            .init_resource::<FactionManager>()
            .init_resource::<FactionResources>()
            .init_resource::<TerritoryMap>()
            .init_resource::<InfrastructureGrid>()
            .init_resource::<TileOperatingGrid>()
            .init_resource::<IndustryGrid>()
            .register_type::<FactionId>()
            .register_type::<DiplomaticRelation>()
            .register_type::<TileTerritory>()
            .register_type::<FactionResources>()
            .add_systems(
                OnEnter(AppState::InGame),
                setup_initial_faction_territories.after(crate::map::generate_hex_map),
            )
            .add_systems(
                Update,
                (
                    infrastructure::update_infrastructure_connectivity,
                    upkeep::process_turn_upkeep_system
                        .after(infrastructure::update_infrastructure_connectivity),
                    industry::process_turn_production_system
                        .after(upkeep::process_turn_upkeep_system),
                    update_territory_overlays,
                    territory::update_city_labels.after(TransformSystems::Propagate),
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnEnter(AppState::Title),
                (cleanup_territories, reset_faction_phase1_data),
            );
    }
}

fn reset_faction_phase1_data(
    mut infra: ResMut<InfrastructureGrid>,
    mut op_grid: ResMut<TileOperatingGrid>,
    mut ind_grid: ResMut<IndustryGrid>,
) {
    infra.edges.clear();
    infra.connectivity.clear();
    op_grid.upkeeps.clear();
    op_grid.states.clear();
    ind_grid.plants.clear();
    ind_grid.advanced_stocks.clear();
}
