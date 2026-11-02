use bevy::prelude::*;
use std::collections::HashMap;

use crate::map::MapGrid;
use crate::map::hex::HexCoord;
use crate::map::terrain::TerrainType;
use crate::turn::AdvanceTurnMessage;
use crate::unit::types::Unit;

use super::types::{AlienNest, MiasmaMap};

/// ターン進行メッセージを受信し、エイリアン生態系（瘴気拡散・刺激ヘイト・ユニット損耗）を更新
pub fn handle_alien_turn_update(
    mut messages: MessageReader<AdvanceTurnMessage>,
    map_grid: Res<MapGrid>,
    mut nests: Query<&mut AlienNest>,
    mut miasma_map: ResMut<MiasmaMap>,
    mut units: Query<(&mut Unit, &Transform)>,
) {
    for _ in messages.read() {
        if map_grid.tiles.is_empty() {
            continue;
        }

        let map_w = map_grid.width.max(1);

        // 1. 周囲の人類ユニット接近による刺激ヘイト蓄積
        let unit_coords: Vec<HexCoord> = units.iter().map(|(u, _)| u.coord).collect();
        for mut nest in &mut nests {
            let mut irritation_delta = 0.0;
            for &u_coord in &unit_coords {
                let dist = nest.coord.distance_with_width(u_coord, map_w);
                if dist <= 1 {
                    irritation_delta += 15.0; // 隣接タイルに駐留
                } else if dist <= 2 {
                    irritation_delta += 5.0;  // 視界・近接圏内
                }
            }

            if irritation_delta > 0.0 {
                nest.add_irritation(irritation_delta);
                info!(
                    "Nest #{} stimulated! Irritation: {:.1}, Phase: {:?}",
                    nest.id, nest.irritation, nest.lifecycle
                );
            } else {
                // 刺激がなければ徐々に沈静化（ターン毎に -2.0）
                nest.add_irritation(-2.0);
            }
        }

        // 2. 瘴気（Miasma）の放出・拡散・自然減衰
        let mut new_concentrations = HashMap::new();

        // 2a. 既存の瘴気から自然減衰と隣接タイルへの拡散を計算
        for (&coord, &conc) in &miasma_map.concentrations {
            let retained = conc * (1.0 - MiasmaMap::DISSIPATION_RATE - MiasmaMap::DIFFUSION_COEFF);
            if retained > 0.01 {
                *new_concentrations.entry(coord).or_insert(0.0) += retained;
            }

            let diff_amount = (conc * MiasmaMap::DIFFUSION_COEFF) / 6.0;
            let neighbors = coord.neighbors_with_width(map_w);
            for n_coord in neighbors {
                // 山岳は瘴気の拡散を遮断する
                if map_grid.terrain_data.get(&n_coord).copied() == Some(TerrainType::Mountains) {
                    continue;
                }
                *new_concentrations.entry(n_coord).or_insert(0.0) += diff_amount;
            }
        }

        // 2b. 活性状態の巣窟からの新瘴気放出
        for nest in &nests {
            let emission = nest.lifecycle.base_miasma_emission();
            *new_concentrations.entry(nest.coord).or_insert(0.0) += emission;
        }

        // 瘴気濃度の適用（クランプと不要エントリの除去）
        miasma_map.concentrations.clear();
        for (coord, conc) in new_concentrations {
            let clamped = conc.clamp(0.0, MiasmaMap::MAX_CONCENTRATION);
            if clamped > 0.01 {
                miasma_map.concentrations.insert(coord, clamped);
            }
        }

        // 3. 瘴気による人類ユニットへの有害影響（HP損耗ダメージ）
        for (mut unit, _) in &mut units {
            let miasma_level = miasma_map.get(&unit.coord);
            if miasma_level >= 0.7 {
                // 高濃度瘴気: 重篤なダメージ（最大HPの10%）
                let dmg = (unit.max_hp as f32 * 0.10).max(5.0) as u32;
                unit.hp = unit.hp.saturating_sub(dmg);
                info!(
                    "Unit {:?} suffered heavy miasma hazard! Remaining HP: {}/{}",
                    unit.group_type, unit.hp, unit.max_hp
                );
            } else if miasma_level >= 0.3 {
                // 中濃度瘴気: 軽微なダメージ（最大HPの3%）
                let dmg = (unit.max_hp as f32 * 0.03).max(2.0) as u32;
                unit.hp = unit.hp.saturating_sub(dmg);
            }
        }
    }
}

/// タイトル画面等への遷移時にエイリアン生態系をリセット
pub fn cleanup_alien_ecosystem(
    mut commands: Commands,
    nests: Query<Entity, With<AlienNest>>,
    nest_models: Query<Entity, With<super::mesh::AlienNestModel>>,
    mut miasma: ResMut<MiasmaMap>,
) {
    for entity in &nest_models {
        commands.entity(entity).despawn();
    }
    for entity in &nests {
        commands.entity(entity).despawn();
    }
    miasma.concentrations.clear();
}
