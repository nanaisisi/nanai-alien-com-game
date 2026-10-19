use bevy::prelude::*;

use crate::faction::FactionOutpost;
use crate::map::MapGrid;

use super::mesh::spawn_unit_model;
use super::types::{CombatGroupType, SelectedUnit, Unit};

/// 初期ゲーム開始時、各派閥の前哨基地周辺に初期戦闘団をスポーンする
pub fn spawn_initial_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_grid: Res<MapGrid>,
    outposts: Query<(&FactionOutpost, &Transform)>,
    existing_units: Query<Entity, With<Unit>>,
) {
    if !existing_units.is_empty() {
        return;
    }

    if outposts.is_empty() || map_grid.tiles.is_empty() {
        return;
    }

    let map_w = map_grid.width.max(1);

    info!("Spawning initial combat groups for all factions...");

    for (outpost, _outpost_transform) in &outposts {
        let faction = outpost.faction;
        let base_coord = outpost.coord;

        // 前哨基地の隣接タイル候補を取得
        let neighbors = base_coord.neighbors_with_width(map_w);

        // 3種類の初期戦闘団
        let initial_groups = [
            CombatGroupType::Scout,
            CombatGroupType::Colonist,
            CombatGroupType::LightInfantry,
        ];

        let mut spawned_coords = Vec::new();

        for (idx, &group_type) in initial_groups.iter().enumerate() {
            // 通行可能な隣接タイルを探す（陸地かつ未重複）
            let mut target_coord = None;
            for n_idx in idx..(idx + 6) {
                let candidate = neighbors[n_idx % 6];
                if let Some(&terrain) = map_grid.terrain_data.get(&candidate)
                    && terrain.is_passable_ground()
                    && !spawned_coords.contains(&candidate)
                {
                    target_coord = Some(candidate);
                    break;
                }
            }

            // 隣接タイルに空き通行可能陸地がない場合、基地タイル自体が陸地かつまだ配置されていなければ検討
            if target_coord.is_none()
                && let Some(&base_terrain) = map_grid.terrain_data.get(&base_coord)
                && base_terrain.is_passable_ground()
                && !spawned_coords.contains(&base_coord)
            {
                target_coord = Some(base_coord);
            }

            // 通行可能タイルが見つかった場合は地表配備、なければ基地内の地下要塞待機（BunkerReserve）とする
            let (deploy_coord, deployment_status) = if let Some(coord) = target_coord {
                spawned_coords.push(coord);
                (coord, super::types::DeploymentStatus::ActiveGarrison)
            } else {
                info!(
                    "No passable ground tile for {:?} ({:?}). Spawning into BunkerReserve at outpost.",
                    group_type, faction
                );
                (
                    base_coord,
                    super::types::DeploymentStatus::BunkerReserve {
                        priority_order: idx as u8,
                    },
                )
            };

            let is_reserve = deployment_status.is_hidden_stack();

            // ユニットエンティティのスポーン
            let terrain_height = map_grid
                .terrain_data
                .get(&deploy_coord)
                .map(|t| t.height())
                .unwrap_or(0.0);

            let world_pos = deploy_coord.to_world_pos(crate::map::HEX_RADIUS);

            let mut unit = Unit::new(faction, group_type, deploy_coord);
            unit.deployment_status = deployment_status;
            if is_reserve {
                unit.is_exhausted = true;
                unit.current_movement = 0;
            }

            let unit_visibility = if is_reserve {
                Visibility::Hidden
            } else {
                Visibility::default()
            };

            let unit_entity = commands
                .spawn((
                    unit,
                    crate::unit::CombatGroupLogistics::default(),
                    Transform::from_xyz(world_pos.x, terrain_height, world_pos.z),
                    unit_visibility,
                ))
                .id();

            // SFプロシージャルメッシュの子階層を追加
            spawn_unit_model(
                &mut commands,
                unit_entity,
                &mut meshes,
                &mut materials,
                faction,
                group_type,
            );
        }
    }
}

/// タイトル画面等への遷移時に全ユニットをクリーンアップ
pub fn cleanup_units(
    mut commands: Commands,
    units: Query<Entity, With<Unit>>,
    mut selected_unit: ResMut<SelectedUnit>,
) {
    selected_unit.0 = None;
    for entity in &units {
        commands.entity(entity).despawn();
    }
}
