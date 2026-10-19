use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use super::hex::{self, HexCoord};
use super::settings::{LandmassType, MapConfig, PlanetEnvironment};
use super::terrain::{TerrainType, WaterSalinity};
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

    let landmass = map_config.landmass;

    info!(
        "Generating Cylindrical Overworld Hex Map (Env: {:?}, Landmass: {:?}, Width: {}, Height: {}, Seed: {})...",
        env, landmass, map_w, map_h, base_seed
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
        TerrainType::Lake,
        TerrainType::ToxicSwamp,
        TerrainType::Jungle,
    ];

    for t in terrain_types {
        let mut mat = StandardMaterial::from_color(t.base_color());
        mat.perceptual_roughness = match t {
            TerrainType::Ocean | TerrainType::Lake => 0.1,
            _ => 0.85,
        };
        mat.reflectance = match t {
            TerrainType::Ocean | TerrainType::Lake => 0.8,
            _ => 0.2,
        };
        mat_cache.insert(t, materials.add(mat));
    }

    // 水域塩分濃度ごとの専用マテリアルキャッシュ
    let mut salinity_mat_cache: HashMap<WaterSalinity, Handle<StandardMaterial>> = HashMap::new();
    for &sal in &[
        WaterSalinity::Fresh,
        WaterSalinity::Brackish,
        WaterSalinity::Saline,
        WaterSalinity::Hypersaline,
    ] {
        let mut mat = StandardMaterial::from_color(sal.color_tint());
        mat.perceptual_roughness = match sal {
            WaterSalinity::Hypersaline => 0.35, // 塩の結晶感・かすかな白光沢
            _ => 0.1,
        };
        mat.reflectance = match sal {
            WaterSalinity::Hypersaline => 0.6,
            _ => 0.8,
        };
        salinity_mat_cache.insert(sal, materials.add(mat));
    }

    // Step 1: 各タイルの標高(elevation)と陸地/海洋マスクを事前計算
    let half_h = map_h / 2;
    let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
    let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();

    // 環境に応じた基礎海水面
    let base_sea_threshold = env.sea_level_threshold();

    // 陸地形状タイプ (Landmass) に応じた調整
    // パンゲアの場合: 大陸中心となる経度をシードから決定
    let pangaea_center_col = (base_seed.wrapping_mul(1103515245) % (map_w as u32)) as f32;

    for row in -half_h..=half_h {
        let lat_ratio = row.abs() as f32 / half_h as f32;
        let lat_factor = lat_ratio.powf(1.8);

        for col in 0..map_w {
            let n = cylinder_noise_with_size(col, row, base_seed, map_w, map_h);

            let (elevation, sea_threshold) = match landmass {
                LandmassType::Continents => {
                    // 標準的な多大陸: 極地低減 + 標準海水面
                    let elev = (n - lat_factor * 0.40).clamp(0.0, 1.0);
                    (elev, base_sea_threshold)
                }
                LandmassType::Pangaea => {
                    // 1つの巨大超大陸: 大陸中心(経度)からの最短円周距離を計算
                    let mut dx = (col as f32 - pangaea_center_col).abs();
                    if dx > (map_w as f32 / 2.0) {
                        dx = map_w as f32 - dx;
                    }
                    let norm_dist_x = dx / (map_w as f32 / 2.0); // 0.0 (中心) 〜 1.0 (対蹠地)
                    let center_dist = (norm_dist_x * norm_dist_x + lat_ratio * lat_ratio).sqrt();
                    // 中心付近を強く盛り上げ、遠方（対蹠地・大洋）を沈める
                    let pangaea_bias = (1.0 - center_dist * 1.35).clamp(-0.6, 0.4);
                    let elev = (n * 0.65 + pangaea_bias * 0.55 + 0.15 - lat_factor * 0.25).clamp(0.0, 1.0);
                    (elev, base_sea_threshold)
                }
                LandmassType::SolidLand => {
                    // 全面陸型: 海水面を大幅に下げ、極地以外はほぼ陸地化（内海や湖水が点在）
                    let elev = (n - lat_factor * 0.20).clamp(0.0, 1.0);
                    (elev, (base_sea_threshold * 0.45).min(0.20))
                }
                LandmassType::Archipelago => {
                    // 島嶼群: 海水面を高めに設定し、細かな起伏で多数の島を浮き上がらせる
                    // 細部ノイズをブレンドして細分化
                    let detail_n = cylinder_noise_with_size(col, row, base_seed.wrapping_add(99991), map_w, map_h);
                    let mixed_n = n * 0.55 + detail_n * 0.45;
                    let elev = (mixed_n - lat_factor * 0.30).clamp(0.0, 1.0);
                    (elev, (base_sea_threshold * 1.15 + 0.08).clamp(0.52, 0.68))
                }
            };

            elevations.insert((col, row), elevation);
            is_land.insert((col, row), elevation >= sea_threshold);
        }
    }

    // Step 2: 水域の連結成分判定（外洋 Ocean と閉鎖水域 Lake の分類）
    // - 南北両極の縁に接している、または水域全体の35%以上を占める連結成分を外洋(Ocean)とする
    // - それ以外の陸地に完全に囲まれた独立水域を湖沼(Lake)とする
    let mut water_visited: HashSet<HexCoord> = HashSet::new();
    let mut is_ocean_tile: HashMap<HexCoord, bool> = HashMap::new();
    let mut total_water_tiles = 0;

    for row in -half_h..=half_h {
        for col in 0..map_w {
            if !is_land[&(col, row)] {
                total_water_tiles += 1;
            }
        }
    }

    let ocean_size_threshold = (total_water_tiles as f32 * 0.35).max(12.0) as usize;

    for row in -half_h..=half_h {
        for col in 0..map_w {
            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            if is_land[&(col, row)] || water_visited.contains(&coord) {
                continue;
            }

            // BFSで水域連結成分を探索
            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            let mut touches_pole = false;

            water_visited.insert(coord);
            queue.push_back(coord);

            while let Some(current) = queue.pop_front() {
                component.push(current);
                let (_cc, cr) = current.to_col_row_with_width(map_w);
                if cr == -half_h || cr == half_h {
                    touches_pole = true;
                }

                for n_coord in current.neighbors_with_width(map_w) {
                    let (nc, nr) = n_coord.to_col_row_with_width(map_w);
                    if nr < -half_h || nr > half_h {
                        touches_pole = true;
                        continue;
                    }
                    if !is_land[&(nc, nr)] && water_visited.insert(n_coord) {
                        queue.push_back(n_coord);
                    }
                }
            }

            let is_ocean = touches_pole || component.len() >= ocean_size_threshold;
            for &c in &component {
                is_ocean_tile.insert(c, is_ocean);
            }
        }
    }

    // Step 2.5: 各水域タイルの塩分濃度 (Salinity) 判定
    // - 外洋(Ocean): 海水 Saline (~35‰)
    // - 閉鎖水域(Lake):
    //   - 乾燥砂漠環境(Arid)または乾燥度の高い内陸窪地: 蒸発濃縮により高塩分・塩湖 Hypersaline (>50‰, 死海/グレートソルト湖型)
    //   - 比較的大規模な内陸湖 (8タイル以上): 堆積・流入により汽水 Brackish (0.5-30‰, カスピ海型)
    //   - 通常規模の内陸湖 (7タイル以下): 流入・循環する純淡水 Fresh (<0.5‰)
    let mut salinity_map: HashMap<HexCoord, WaterSalinity> = HashMap::new();
    let mut lake_component_visited: HashSet<HexCoord> = HashSet::new();

    for row in -half_h..=half_h {
        for col in 0..map_w {
            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            if is_land[&(col, row)] {
                continue;
            }

            if is_ocean_tile.get(&coord).copied().unwrap_or(true) {
                salinity_map.insert(coord, WaterSalinity::Saline);
                continue;
            }

            if lake_component_visited.contains(&coord) {
                continue;
            }

            // 内陸湖コンポーネントを探索
            let mut lake_comp = Vec::new();
            let mut queue = VecDeque::new();
            lake_component_visited.insert(coord);
            queue.push_back(coord);

            let mut avg_moisture = 0.0;

            while let Some(current) = queue.pop_front() {
                lake_comp.push(current);
                let (cc, cr) = current.to_col_row_with_width(map_w);
                let moist = cylinder_noise_with_size(cc, cr, base_seed.wrapping_add(8888), map_w, map_h);
                avg_moisture += moist;

                for n_coord in current.neighbors_with_width(map_w) {
                    let (nc, nr) = n_coord.to_col_row_with_width(map_w);
                    if nr >= -half_h && nr <= half_h && !is_land[&(nc, nr)] && !is_ocean_tile.get(&n_coord).copied().unwrap_or(true) && lake_component_visited.insert(n_coord) {
                        queue.push_back(n_coord);
                    }
                }
            }

            if !lake_comp.is_empty() {
                avg_moisture /= lake_comp.len() as f32;
            }

            let salinity = if env == PlanetEnvironment::Arid || avg_moisture < 0.28 {
                // 極度の乾燥または砂漠気候の内陸湖 -> 塩湖 (死海・アラル海・塩原)
                WaterSalinity::Hypersaline
            } else if lake_comp.len() >= 8 {
                // 巨大な内陸閉鎖水域 -> 汽水 (カスピ海型)
                WaterSalinity::Brackish
            } else {
                // 通常の内陸湖沼 -> 淡水 (バイカル湖・五大湖型)
                WaterSalinity::Fresh
            };

            for c in lake_comp {
                salinity_map.insert(c, salinity);
            }
        }
    }

    // Step 3: 地形タイプの決定
    let mut terrain_map: HashMap<HexCoord, TerrainType> = HashMap::new();
    let mountain_threshold = env.mountain_threshold();
    let hill_threshold = env.hill_threshold();

    for row in -half_h..=half_h {
        for col in 0..map_w {
            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            let land = is_land[&(col, row)];

            if !land {
                let is_ocean = is_ocean_tile.get(&coord).copied().unwrap_or(true);
                let water_type = if is_ocean {
                    TerrainType::Ocean
                } else {
                    TerrainType::Lake
                };
                terrain_map.insert(coord, water_type);
                continue;
            }

            let elev = elevations[&(col, row)];
            let moisture =
                cylinder_noise_with_size(col, row, base_seed.wrapping_add(8888), map_w, map_h);
            let detail =
                cylinder_noise_with_size(col, row, base_seed.wrapping_add(54321), map_w, map_h);

            // 隣接セルに水域（外洋または湖）があるかチェック (海岸線判定)
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
                            let salinity = salinity_map.get(&coord).copied();
                            let material_handle = if let Some(sal) = salinity {
                                salinity_mat_cache.get(&sal).unwrap().clone()
                            } else {
                                mat_cache.get(&terrain).unwrap().clone()
                            };

                            let tile_entity = section
                                .spawn((
                                    HexTile { coord, terrain, salinity },
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
                                if let Some(sal) = salinity {
                                    map_grid.water_salinity.insert(coord, sal);
                                }
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
    map_grid.water_salinity.clear();
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

    #[test]
    fn test_lake_and_ocean_classification() {
        let (w, h) = (20, 10);
        let half_h = h / 2;

        // すべて陸地、ただし中央に 1 タイルだけ水域（完全な孤立湖）
        // および最北端 row = -half_h に水域（外洋）
        let mut is_land = HashMap::new();
        for row in -half_h..=half_h {
            for col in 0..w {
                is_land.insert((col, row), true);
            }
        }

        // 孤立湖
        is_land.insert((5, 0), false);
        // 極地海洋
        is_land.insert((0, -half_h), false);

        let mut water_visited: HashSet<HexCoord> = HashSet::new();
        let mut is_ocean_tile: HashMap<HexCoord, bool> = HashMap::new();
        let total_water = 2;
        let ocean_size_threshold = (total_water as f32 * 0.35).max(12.0) as usize;

        for row in -half_h..=half_h {
            for col in 0..w {
                let coord = HexCoord::from_col_row_with_width(col, row, w);
                if is_land[&(col, row)] || water_visited.contains(&coord) {
                    continue;
                }

                let mut component = Vec::new();
                let mut queue = VecDeque::new();
                let mut touches_pole = false;

                water_visited.insert(coord);
                queue.push_back(coord);

                while let Some(current) = queue.pop_front() {
                    component.push(current);
                    let (_cc, cr) = current.to_col_row_with_width(w);
                    if cr == -half_h || cr == half_h {
                        touches_pole = true;
                    }

                    for n_coord in current.neighbors_with_width(w) {
                        let (nc, nr) = n_coord.to_col_row_with_width(w);
                        if nr < -half_h || nr > half_h {
                            touches_pole = true;
                            continue;
                        }
                        if !is_land[&(nc, nr)] && water_visited.insert(n_coord) {
                            queue.push_back(n_coord);
                        }
                    }
                }

                let is_ocean = touches_pole || component.len() >= ocean_size_threshold;
                for c in component {
                    is_ocean_tile.insert(c, is_ocean);
                }
            }
        }

        let lake_coord = HexCoord::from_col_row_with_width(5, 0, w);
        let ocean_coord = HexCoord::from_col_row_with_width(0, -half_h, w);

        assert_eq!(is_ocean_tile.get(&lake_coord), Some(&false), "Inland isolated water should be classified as Lake (freshwater)");
        assert_eq!(is_ocean_tile.get(&ocean_coord), Some(&true), "Polar/large water body should be classified as Ocean (saltwater)");
    }

    #[test]
    fn test_water_salinity_distinction() {
        use super::super::terrain::WaterSalinity;

        // 産出力の検証
        let fresh_out = WaterSalinity::Fresh.base_output();
        assert_eq!(fresh_out.biomass, 2.0);
        assert_eq!(fresh_out.energy, 0.0);

        let brackish_out = WaterSalinity::Brackish.base_output();
        assert_eq!(brackish_out.biomass, 1.5);
        assert_eq!(brackish_out.energy, 0.5);

        let saline_out = WaterSalinity::Saline.base_output();
        assert_eq!(saline_out.biomass, 1.0);
        assert_eq!(saline_out.energy, 1.0);

        let hyper_out = WaterSalinity::Hypersaline.base_output();
        assert_eq!(hyper_out.biomass, 0.0);
        assert_eq!(hyper_out.ore, 2.0);
    }
}
