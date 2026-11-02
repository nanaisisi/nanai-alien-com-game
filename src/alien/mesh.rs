use bevy::prelude::*;

/// エイリアン巣窟モデルのマーカーコンポーネント
#[derive(Component)]
pub struct AlienNestModel;

/// エイリアン巣窟のプロシージャル3Dモデル（有機的バイオマウンド・尖塔）を生成する
pub fn spawn_nest_model(
    commands: &mut Commands,
    parent_entity: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let base_color = Color::srgb(0.55, 0.15, 0.45); // 紫紅色のエイリアン生体色
    let glow_color = Color::srgb(0.75, 0.20, 0.60);

    let base_material = materials.add(StandardMaterial {
        base_color,
        perceptual_roughness: 0.6,
        metallic: 0.1,
        reflectance: 0.2,
        ..default()
    });

    let bulb_material = materials.add(StandardMaterial {
        base_color: glow_color,
        emissive: LinearRgba::rgb(0.3, 0.05, 0.25),
        perceptual_roughness: 0.3,
        metallic: 0.0,
        ..default()
    });

    let base_mesh = meshes.add(Cone::new(3.2, 3.5));
    let central_spire_mesh = meshes.add(Cylinder::new(0.9, 5.0));
    let bulb_mesh = meshes.add(Sphere::new(1.1));

    commands.entity(parent_entity).with_children(|parent| {
        // 土台の生体マウンド
        parent.spawn((
            Mesh3d(base_mesh),
            MeshMaterial3d(base_material.clone()),
            Transform::from_xyz(0.0, 1.75, 0.0),
            AlienNestModel,
        ));

        // 中央の有機的尖塔
        parent.spawn((
            Mesh3d(central_spire_mesh),
            MeshMaterial3d(base_material),
            Transform::from_xyz(0.0, 3.0, 0.0),
            AlienNestModel,
        ));

        // 頂上の発光嚢胞（胞子放出器官）
        parent.spawn((
            Mesh3d(bulb_mesh),
            MeshMaterial3d(bulb_material),
            Transform::from_xyz(0.0, 5.5, 0.0),
            AlienNestModel,
        ));
    });
}
