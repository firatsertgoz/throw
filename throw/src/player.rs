use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

const PLAYER_SPEED: f32 = 5.0;

#[derive(Debug, Component)]
pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
        app.add_systems(Startup, spawn_player);
    }
}

fn move_player(
    mut controllers: Query<&mut KinematicCharacterController>,
    mut character_query: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for mut player_transform in controllers.iter_mut() {
        let cam = match camera_query.get_single() {
            Ok(cam) => cam,
            Err(_) => return,
        };
        let mut to_move = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            to_move += cam.forward();
        }
        if keyboard.pressed(KeyCode::S) {
            to_move += cam.back();
        }
        if keyboard.pressed(KeyCode::A) {
            to_move += cam.left();
        }
        if keyboard.pressed(KeyCode::D) {
            to_move += cam.right();
        }

        let movement = to_move.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
        to_move.y = 0.0;
        player_transform.translation = Some(movement);

        if movement.length_squared() > 0.0 {
            character_query
                .get_single_mut()
                .unwrap()
                .look_to(-to_move, Vec3::Y);
        }
    }
}
fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        SceneBundle {
            scene: assets.load("Ninja-xGYmeDpfTu.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.1, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
    );

    commands
        .spawn(KinematicCharacterController {
            custom_mass: Some(1.0),
            ..default()
        })
        .insert(Collider::cuboid(10.0, 0.0, 10.0))
        .insert(Restitution::coefficient(0.1))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(player);
}
