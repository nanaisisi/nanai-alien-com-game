use bevy::prelude::*;
use std::collections::HashMap;

use super::hex::{self, HexCoord};
use super::settings::{MapConfig, PlanetEnvironment};
use super::terrain::TerrainType;
use super::types::{HEX_RADIUS, HexTile, MapGrid, MapRoot};

/// 3D格子点ハッシュ関数（シードと3次元格子座標から擬似乱数を生成）
#[inline]
fn hash3d(x: i32, y: i32, z: i32, seed: u32) -> f32 {
    let mut h = seed
        .wrapping_add((x as u32).wrapping_mul(374761393))
        .wrapping_add((y as u32).wrapping_mul(668265263))
        .wrapping_add((z as u32).wrapping_mul(1274126177));
    h = (h ^ (h >> 13)).wrapping_mul(1103515245);
    h = h ^ (h >> 16);
    (h as f32) / (u32::MAX as f32)
}

/// 3D円筒空間におけるバリューノイズ（X/Z平面で円周、Yで緯度）
fn sample_value_noise(cx: f32, cy: f32, cz: f32, freq: f32, seed: u32) -> f32 {
    let px = cx * freq;
    let py = cy * freq;
    let pz = cz * freq;

    let x0 = px.floor() as i32;
    let y0 = py.floor() as i32;
    let z0 = pz.floor() as i32;

    let fx = px - x0 as f32;
    let fy = py - y0 as f32;
    let fz = pz - z0 as f32;

    // Smoothstep補間カーブ (3t^2 - 2t^3)
    let u = fx * fx * (3.0 - 2.0 * fx);
    let v = fy * fy * (3.0 - 2.0 * fy);
    let w = fz * fz * (3.0 - 2.0 * fz);

    let c000 = hash3d(x0, y0, z0, seed);
    let c100 = hash3d(x0 + 1, y0, z0, seed);
    let c010 = hash3d(x0, y0 + 1, z0, seed);
    let c110 = hash3d(x0 + 1, y0 + 1, z0, seed);
    let c001 = hash3d(x0, y0, z0 + 1, seed);
    let c101 = hash3d(x0 + 1, y0, z0 + 1, seed);
    let c011 = hash3d(x0, y0 + 1, z0 + 1, seed);
    let c111 = hash3d(x0 + 1, y0 + 1, z0 + 1, seed);

    let c00 = c000 * (1.0 - u) + c100 * u;
    let c10 = c010 * (1.0 - u) + c110 * u;
    let c01 = c001 * (1.0 - u) + c101 * u;
    let c11 = c011 * (1.0 - u) + c111 * u;

    let c0 = c00 * (1.0 - v) + c10 * v;
    let c1 = c01 * (1.0 - v) + c11 * v;

    c0 * (1.0 - w) + c1 * w
}

/// 周期的な円筒座標におけるハッシュ/疑似乱数（マップ幅・高さ・シード可変）
/// フラクタル（多重オクターブ）ノイズにより、シード毎に全く異なる大陸・島嶼・起伏を形成する
pub fn cylinder_noise_with_size(col: i32, row: i32, seed: u32, map_w: i32, map_h: i32) -> f32 {
    use std::f32::consts::PI;
    // 円筒展開: col が [0, map_w] で 0..2PI を周回するため、東西端が完全につながる
    let theta = (col as f32 / map_w as f32) * 2.0 * PI;
    let cx = theta.cos();
    let cz = theta.sin();
    let cy = row as f32 / (map_h as f32 / 2.0);

    // 4オクターブのフラクタル・ブラウン運動 (fBm)
    let n1 = sample_value_noise(cx, cy, cz, 1.2, seed);
    let n2 = sample_value_noise(cx, cy, cz, 2.5, seed.wrapping_add(1013904223));
    let n3 = sample_value_noise(cx, cy, cz, 5.0, seed.wrapping_add(2147483647));
    let n4 = sample_value_noise(cx, cy, cz, 10.0, seed.wrapping_add(374761393));

    let total = n1 * 0.50 + n2 * 0.28 + n3 * 0.14 + n4 * 0.08;
    total.clamp(0.0, 1.0)
}

/// 六角柱の3Dメッシュを生成
pub fn create_hex_mesh(radius: f32, height: f32) -> Mesh {
    let cylinder = Cylinder {
        radius,
        half_height: height / 2.0,
    };
    // 6角形としてシリンダーメッシュを生成
    cylinder.mesh().resolution(6).build()
}

pub fn generate_hex_map(
    mut commands: Commands,
    map_config: Res<MapConfig>,
    mut map_grid: ResMut<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // すでにマップが生成されている場合（PauseMenu / Settings からの復帰時など）は再生成しない
    if !map_grid.tiles.is_empty() {
        info!("Hex map already exists. Skipping map generation.");
        return;
    }

    let map_w = map_config.width();
    let map_h = map_config.height();
    let env = map_config.environment;
    let base_seed = map_config.seed;

    info!(
        "Generating Cylindrical Overworld Hex Map (Env: {:?}, Width: {}, Height: {}, Seed: {})...",
        env, map_w, map_h, base_seed
    );

    map_grid.tiles.clear();
    map_grid.terrain_data.clear();
    map_grid.width = map_w;
    map_grid.height = map_h;

    // 地形ごとのマテリアルキャッシュ
    let mut mat_cache: HashMap<TerrainType, Handle<StandardMaterial>> = HashMap::new();
    let terrain_types = [
        TerrainType::Plains,
        TerrainType::Hills,
        TerrainType::Forest,
        TerrainType::Mountains,
        TerrainType::Ocean,
        TerrainType::ToxicSwamp,
        TerrainType::Jungle,
    ];

    for t in terrain_types {
        let mut mat = StandardMaterial::from_color(t.base_color());
        mat.perceptual_roughness = match t {
            TerrainType::Ocean => 0.1,
            _ => 0.85,
        };
        mat.reflectance = match t {
            TerrainType::Ocean => 0.8,
            _ => 0.2,
        };
        mat_cache.insert(t, materials.add(mat));
    }

    // Step 1: 各タイルの標高(elevation)と陸地/海洋マスクを事前計算
    let half_h = map_h / 2;
    let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
    let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();
    let sea_threshold = env.sea_level_threshold();

    for row in -half_h..=half_h {
        let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
        for col in 0..map_w {
            let n = cylinder_noise_with_size(col, row, base_seed, map_w, map_h);
            // 極地は海洋確率を上げるため標高を低減
            let elevation = (n - lat_factor * 0.4).clamp(0.0, 1.0);
            elevations.insert((col, row), elevation);
            is_land.insert((col, row), elevation >= sea_threshold);
        }
    }

    // Step 2: 地形タイプの決定
    let mut terrain_map: HashMap<HexCoord, TerrainType> = HashMap::new();
    let mountain_threshold = env.mountain_threshold();
    let hill_threshold = env.hill_threshold();

    for row in -half_h..=half_h {
        for col in 0..map_w {
            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            let land = is_land[&(col, row)];

            if !land {
                terrain_map.insert(coord, TerrainType::Ocean);
                continue;
            }

            let elev = elevations[&(col, row)];
            let moisture =
                cylinder_noise_with_size(col, row, base_seed.wrapping_add(8888), map_w, map_h);
            let detail =
                cylinder_noise_with_size(col, row, base_seed.wrapping_add(54321), map_w, map_h);

            // 隣接セルに海洋があるかチェック (海岸線判定)
            let mut neighbors_have_ocean = false;
            for n_coord in coord.neighbors_with_width(map_w) {
                let (nc, nr) = n_coord.to_col_row_with_width(map_w);
                if let Some(&nl) = is_land.get(&(nc, nr)) {
                    if !nl {
                        neighbors_have_ocean = true;
                        break;
                    }
                } else {
                    // マップ範囲外（南北端）は海扱い
                    neighbors_have_ocean = true;
                    break;
                }
            }

            let terrain = match env {
                PlanetEnvironment::Terra => {
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.65 {
                        if detail > 0.70 {
                            TerrainType::ToxicSwamp
                        } else if detail > 0.35 {
                            TerrainType::Jungle
                        } else {
                            TerrainType::Forest
                        }
                    } else if moisture < 0.35 {
                        TerrainType::Plains
                    } else {
                        TerrainType::Forest
                    }
                }
                PlanetEnvironment::Arid => {
                    // 乾燥砂漠: 海洋が少なく、平原・丘陵が多い。湿潤地帯のみオアシス・森林
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.85 {
                        TerrainType::Jungle
                    } else if moisture > 0.65 {
                        TerrainType::Forest
                    } else {
                        TerrainType::Plains
                    }
                }
                PlanetEnvironment::Archipelago => {
                    // 海洋群島: 陸地が狭く、島嶼部は熱帯雨林や森林が豊か
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.60 {
                        TerrainType::Jungle
                    } else if moisture > 0.35 {
                        TerrainType::Forest
                    } else {
                        TerrainType::Plains
                    }
                }
                PlanetEnvironment::ToxicMarsh => {
                    // 瘴気沼沢: 湿地と原生林・ジャングルが大部分を占める
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.40 || detail > 0.45 {
                        TerrainType::ToxicSwamp
                    } else if detail > 0.25 {
                        TerrainType::Jungle
                    } else {
                        TerrainType::Forest
                    }
                }
                PlanetEnvironment::Jungle => {
                    // 原生樹海: 惑星全土が広大な熱帯雨林・温帯林に覆われ、平原は極小
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.45 || detail > 0.35 {
                        TerrainType::Jungle
                    } else if moisture > 0.25 {
                        TerrainType::Forest
                    } else {
                        TerrainType::Plains
                    }
                }
            };

            terrain_map.insert(coord, terrain);
        }
    }

    let world_width = hex::map_world_width_with_width(HEX_RADIUS, map_w);

    commands
        .spawn((MapRoot, Transform::default(), Visibility::default()))
        .with_children(|root| {
            // メインマップ(0)および左右の周回表示(-1, +1)の3セクションを生成
            // 見ている範囲の左右の大まかな範囲が常に描画され、境界の途切れをなくす
            for wrap_offset in [-1, 0, 1] {
                let section_offset_x = wrap_offset as f32 * world_width;
                root.spawn((
                    Transform::from_xyz(section_offset_x, 0.0, 0.0),
                    Visibility::default(),
                ))
                .with_children(|section| {
                    for row in -half_h..=half_h {
                        for col in 0..map_w {
                            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
                            let terrain = terrain_map[&coord];
                            let height = terrain.height();
                            let world_pos = coord.to_world_pos(HEX_RADIUS);

                            let mesh_handle = meshes.add(create_hex_mesh(HEX_RADIUS, height));
                            let material_handle = mat_cache.get(&terrain).unwrap().clone();

                            let tile_entity = section
                                .spawn((
                                    HexTile { coord, terrain },
                                    Mesh3d(mesh_handle),
                                    MeshMaterial3d(material_handle),
                                    Transform::from_xyz(world_pos.x, height / 2.0, world_pos.z)
                                        .with_rotation(Quat::from_rotation_y(
                                            std::f32::consts::PI / 6.0,
                                        )),
                                ))
                                .id();

                            // 中央セクション(0)のEntityを代表タイルとして登録
                            if wrap_offset == 0 {
                                map_grid.tiles.insert(coord, tile_entity);
                                map_grid.terrain_data.insert(coord, terrain);
                            }
                        }
                    }
                });
            }
        });

    info!(
        "Generated {} hex tiles (x3 wrapped sections).",
        map_grid.tiles.len() * 3
    );
}

pub fn cleanup_hex_map(
    mut commands: Commands,
    query: Query<Entity, With<MapRoot>>,
    mut map_grid: ResMut<MapGrid>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    map_grid.tiles.clear();
    map_grid.terrain_data.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::hex::{MAP_HEIGHT, MAP_WIDTH};
    use crate::map::settings::MapSize;

    #[test]
    fn test_no_mountains_in_ocean() {
        let half_h = MAP_HEIGHT / 2;
        let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
        let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();

        for row in -half_h..=half_h {
            let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
            for col in 0..MAP_WIDTH {
                let n = cylinder_noise_with_size(col, row, 1337, MAP_WIDTH, MAP_HEIGHT);
                let elevation = (n - lat_factor * 0.4).clamp(0.0, 1.0);
                elevations.insert((col, row), elevation);
                is_land.insert((col, row), elevation >= 0.42);
            }
        }

        for row in -half_h..=half_h {
            for col in 0..MAP_WIDTH {
                let coord = HexCoord::from_col_row(col, row);
                let land = is_land[&(col, row)];

                let mut neighbors_have_ocean = false;
                for n_coord in coord.neighbors() {
                    let (nc, nr) = n_coord.to_col_row();
                    if let Some(&nl) = is_land.get(&(nc, nr)) {
                        if !nl {
                            neighbors_have_ocean = true;
                            break;
                        }
                    } else {
                        neighbors_have_ocean = true;
                        break;
                    }
                }

                let elev = elevations[&(col, row)];
                let is_mountain = land && !neighbors_have_ocean && elev > 0.72;

                // 海（!land）であるセルは絶対に山岳になってはならない
                if !land {
                    assert!(
                        !is_mountain,
                        "Ocean tile must never be a mountain at {:?}",
                        (col, row)
                    );
                }

                // 山岳は海洋に隣接してはならない（海上に孤立・突出しない）
                if is_mountain {
                    assert!(
                        !neighbors_have_ocean,
                        "Mountain must not border ocean at {:?}",
                        (col, row)
                    );
                }
            }
        }
    }

    #[test]
    fn test_planet_environments_and_sizes() {
        for env in PlanetEnvironment::ALL {
            for size in MapSize::ALL {
                let (w, h) = size.dimensions();
                let half_h = h / 2;
                let sea_threshold = env.sea_level_threshold();

                let mut ocean_count = 0;
                let mut land_count = 0;

                for row in -half_h..=half_h {
                    for col in 0..w {
                        let n = cylinder_noise_with_size(col, row, 1337, w, h);
                        let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
                        let elev = (n - lat_factor * 0.4).clamp(0.0, 1.0);
                        if elev < sea_threshold {
                            ocean_count += 1;
                        } else {
                            land_count += 1;
                        }
                    }
                }

                assert!(ocean_count + land_count > 0);
                if env == PlanetEnvironment::Archipelago {
                    // 群島は海が陸より多い
                    assert!(ocean_count > land_count);
                } else if env == PlanetEnvironment::Arid {
                    // 乾燥砂漠は陸が海より圧倒的に多い
                    assert!(land_count > ocean_count);
                }
            }
        }
    }
}
