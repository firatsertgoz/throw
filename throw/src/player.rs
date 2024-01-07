use bevy::prelude::*;
use bevy_gltf_blueprints::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

const PLAYER_SPEED: f32 = 5.0;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Monster;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        app.register_type::<Monster>();
        app.add_systems(Update, move_player);
        //app.add_systems(Update, spawn_blueprint);
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
        if keyboard.pressed(KeyCode::Space) {
            to_move += cam.up();
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
