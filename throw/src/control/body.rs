use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Component)]
pub struct Body;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        //  app.add_systems(Startup, setup_spine);
    }
}

fn setup_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn setup_head(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn setup_legs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn setup_left_leg(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}
fn setup_right_leg(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn setup_arms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn setup_left_arm(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}
fn setup_right_arm(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn setup_spine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::capsule(
    //         Vec3::new(10.0, 10.0, 10.0),
    //         Vec3::new(0.0, 0.0, 0.0),
    //         3.0,
    //     ))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
    //     .insert(GravityScale(1.0))
    //     // Adds Transform and GlobalTransform components
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
    //     // Adds movement to our dynamic body
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
    //     // Changes the scale of our gravity for this entity
    //     .insert(GravityScale(2.0));
}
