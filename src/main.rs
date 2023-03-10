use bevy::{
    prelude::*
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..10{
    // single Sphere
    let looper: f32 = x as f32;
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {radius: 0.5, sectors: 10, stacks: 10})),
        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(looper, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Capsule{radius: 0.1,
                                                   rings:2,
                                                   depth: 1.0,
                                                   latitudes: 4,
                                                longitudes: 4})),
        material: materials.add(Color::rgb(0.1,0.1,0.1).into()),
        transform: Transform { translation: (0.4,0.4, 0.4), rotation: (), scale: () },
        ..default()
    });
    }
    /**
    pub struct Capsule {
    /// Radius on the `XZ` plane.
    pub radius: f32,
    /// Number of sections in cylinder between hemispheres.
    pub rings: usize,
    /// Height of the middle cylinder on the `Y` axis, excluding the hemispheres.
    pub depth: f32,
    /// Number of latitudes, distributed by inclination. Must be even.
    pub latitudes: usize,
    /// Number of longitudes, or meridians, distributed by azimuth.
    pub longitudes: usize,
    /// Manner in which UV coordinates are distributed vertically.
    pub uv_profile: CapsuleUvProfile,
}
     **/
   // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

