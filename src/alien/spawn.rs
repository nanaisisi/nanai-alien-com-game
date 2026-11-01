use bevy::prelude::*;

use crate::faction::FactionOutpost;
use crate::map::MapGrid;
use crate::map::hex::HexCoord;
use crate::map::terrain::TerrainType;

use super::mesh::spawn_nest_model;
use super::types::{AlienNest, MiasmaMap, NestLifecycle};

/// マップ生成後、各勢力拠点から一定距離離れた適した陸地に巣窟（Nest）をプロシージャル配置する
pub fn spawn_alien_nests(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_grid: Res<MapGrid>,
    outposts: Query<&FactionOutpost>,
    existing_nests: Query<Entity, With<AlienNest>>,
    mut miasma_map: ResMut<MiasmaMap>,
) {
    if !existing_nests.is_empty() {
        return;
    }

    if map_grid.tiles.is_empty() || outposts.is_empty() {
        return;
    }

    info!("Spawning Alien Nests and initial Miasma distribution...");

    let outpost_coords: Vec<HexCoord> = outposts.iter().map(|o| o.coord).collect();
    let map_w = map_grid.width.max(1);

    // 巣窟配置候補タイルの収集
    // 条件:
    // 1. 陸地かつ通過可能（毒性湿地、密林、または平地・丘陵・森林）
    // 2. いずれの勢力前哨基地からも距離4以上離れている
    let mut candidates = Vec::new();

    for (&coord, &terrain) in &map_grid.terrain_data {
        if !terrain.is_passable_ground() && terrain != TerrainType::ToxicSwamp {
            continue;
        }

        // 全拠点からの最小距離
        let min_dist = outpost_coords
            .iter()
            .map(|&o_coord| coord.distance_with_width(o_coord, map_w))
            .min()
            .unwrap_or(0);

        if min_dist >= 4 {
            // 毒性湿地や密林にはより高い重み
            let weight = match terrain {
                TerrainType::ToxicSwamp => 10,
                TerrainType::Jungle => 7,
                TerrainType::Forest => 4,
                _ => 2,
            };
            candidates.push((coord, weight));
        }
    }

    // 候補から分散して数箇所（マップサイズに応じて 4〜8 個程度）選択
    let target_nest_count = ((map_grid.width * map_grid.height) / 250).clamp(4, 8);
    let mut chosen_nest_coords: Vec<HexCoord> = Vec::new();

    // 簡易決定論的・距離分散選択
    candidates.sort_by_key(|&(coord, weight)| {
        ((coord.q * 37 + coord.r * 17).abs() % 100) - weight
    });

    for (candidate, _) in candidates {
        if chosen_nest_coords.len() >= target_nest_count as usize {
            break;
        }

        let too_close = chosen_nest_coords
            .iter()
            .any(|&chosen| candidate.distance_with_width(chosen, map_w) < 5);

        if !too_close {
            chosen_nest_coords.push(candidate);
        }
    }

    // 巣窟エンティティの生成と初期瘴気の展開
    for (id, &coord) in chosen_nest_coords.iter().enumerate() {
        let mut nest = AlienNest::new((id + 1) as u32, coord);
        // 最初から1つはActive状態にして刺激を与える
        if id == 0 {
            nest.lifecycle = NestLifecycle::Active;
            nest.irritation = 30.0;
        }

        let terrain_height = map_grid
            .terrain_data
            .get(&coord)
            .map(|t| t.height())
            .unwrap_or(0.0);

        let world_pos = coord.to_world_pos(crate::map::HEX_RADIUS);

        // 巣窟タイルおよび周囲に初期瘴気を散布
        let initial_miasma = match nest.lifecycle {
            NestLifecycle::Active => 0.6,
            _ => 0.3,
        };
        miasma_map.set(coord, initial_miasma);

        let neighbors = coord.neighbors_with_width(map_w);
        for neighbor in neighbors {
            miasma_map.add(neighbor, initial_miasma * 0.4);
        }

        let nest_entity = commands
            .spawn((
                nest,
                Transform::from_xyz(world_pos.x, terrain_height, world_pos.z),
                Visibility::default(),
            ))
            .id();

        spawn_nest_model(
            &mut commands,
            nest_entity,
            &mut meshes,
            &mut materials,
        );
    }

    info!(
        "Successfully spawned {} alien nests across the map.",
        chosen_nest_coords.len()
    );
}
