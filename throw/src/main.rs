mod camera;
mod player;

use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

use camera::CameraPlugin;
use player::PlayerPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        // .add_systems(Update, move_player)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(floor);
}
