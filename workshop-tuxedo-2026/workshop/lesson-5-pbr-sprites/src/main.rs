use bevy::core_pipeline::Skybox;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;

/// Asset prefix used for all sprite textures.
/// Expects `{PREFIX}diffuse.png`, `{PREFIX}normal.png`, `{PREFIX}combined.png` in `assets/`.
const SPRITE_PREFIX: &str = "ball";

/// PBR roughness used for all PBR-lit sprites. 0.0 = mirror, 1.0 = fully rough/diffuse.
const PERCEPTUAL_ROUGHNESS: f32 = 0.2;

#[derive(Component)]
struct OrbitingLight;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_light)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera with environment map (provides reflections for metallic surfaces) and skybox
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 1000.0,
            ..default()
        },
        Skybox {
            image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            brightness: 1000.0,
            ..default()
        },
    ));

    // Point light that orbits the sprites
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(3.0, 0.0, 2.0),
        OrbitingLight,
    ));

    // Shared quad mesh facing the camera (+Z normal). Tangents are required for normal maps.
    let mut quad_mesh = Mesh::from(Plane3d::new(Vec3::Z, Vec2::ONE).mesh().size(2.0, 2.0));
    quad_mesh.generate_tangents().unwrap();
    let mesh = meshes.add(quad_mesh);

    // Build asset paths from the prefix
    let diffuse_path = format!("{SPRITE_PREFIX}diffuse.png");
    let normal_path = format!("{SPRITE_PREFIX}normal.png");
    let combined_path = format!("{SPRITE_PREFIX}combined.png");

    // --- TEST: Reflective metal sphere (no textures) above the row ---
    // If reflections don't show on this, the environment map isn't loading.
    let test_sphere_mesh = meshes.add(Mesh::from(Sphere::new(0.8).mesh().ico(5).unwrap()));
    let test_sphere_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 1.0,
        perceptual_roughness: 0.05,
        ..default()
    });
    commands.spawn((
        Mesh3d(test_sphere_mesh),
        MeshMaterial3d(test_sphere_material),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));

    // --- 1: PBR sprite with diffuse + normal map ---
    let pbr_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(&diffuse_path)),
        normal_map_texture: Some(asset_server.load(&normal_path)),
        perceptual_roughness: PERCEPTUAL_ROUGHNESS,
        metallic: 0.0,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(pbr_material),
        Transform::from_xyz(-5.0, 0.0, 0.0),
        NotShadowCaster,
    ));

    // --- 2: Non-PBR sprite (unlit, fully baked lighting) ---
    let unlit_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(&combined_path)),
        unlit: true,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(unlit_material),
        Transform::from_xyz(-2.5, 0.0, 0.0),
        NotShadowCaster,
    ));

    // --- 3: Diffuse-only reference (no normal map, with PBR lighting) ---
    let diffuse_only_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(&diffuse_path)),
        perceptual_roughness: PERCEPTUAL_ROUGHNESS,
        metallic: 0.0,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(diffuse_only_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        NotShadowCaster,
    ));

    // --- 4: Combined (pre-lit) albedo + normal map, with PBR lighting ---
    let combined_normal_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(&combined_path)),
        normal_map_texture: Some(asset_server.load(&normal_path)),
        perceptual_roughness: PERCEPTUAL_ROUGHNESS,
        metallic: 0.0,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(combined_normal_material),
        Transform::from_xyz(2.5, 0.0, 0.0),
        NotShadowCaster,
    ));

    // --- 5: PBR with diffuse + normal map, MAX metallic ---
    let metallic_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(&diffuse_path)),
        normal_map_texture: Some(asset_server.load(&normal_path)),
        perceptual_roughness: PERCEPTUAL_ROUGHNESS,
        metallic: 1.0,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(metallic_material),
        Transform::from_xyz(5.0, 0.0, 0.0),
        NotShadowCaster,
    ));
}

fn orbit_light(mut query: Query<&mut Transform, With<OrbitingLight>>, time: Res<Time>) {
    let t = time.elapsed_secs();
    for mut transform in query.iter_mut() {
        transform.translation.x = t.cos() * 3.0;
        transform.translation.y = t.sin() * 3.0;
        transform.translation.z = 2.0;
    }
}
