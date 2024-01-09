use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::crossbeam::epoch::Pointable};

use crate::camera::CameraTrackable;

#[derive(Debug, Component)]
pub struct Body;

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
//TODO: Add Spherical Joint to connect the arm to the torso

fn setup_left_arm(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

//TODO: Add Spherical Joint to connect the arm to the torso
fn setup_right_arm(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(RigidBody::Dynamic).insert((
        Collider::capsule(
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            0.25,
        ),
        CameraTrackable,
    ));
}

pub(crate) fn setup_spine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(RigidBody::Dynamic).insert((
        Collider::capsule(
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            0.25,
        ),
        CameraTrackable,
    ));
}

pub(crate) fn setup_torso(mut commands: Commands) {
    // Attach a single collider to a rigid-body.
    commands.spawn(RigidBody::Dynamic).insert((
        Collider::capsule(
            Vec3 {
                x: 0.0,
                y: 3.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        ),
        CameraTrackable,
    ));

    // Attach a multiple colliders to a rigid-body.
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .with_children(|children| {
    //         children
    //             .spawn(Collider::ball(0.5))
    //             // Position the collider relative to the rigid-body.
    //             .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -1.0)));
    //         children
    //             .spawn(Collider::ball(0.5))
    //             // Position the collider relative to the rigid-body.
    //             .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));
    //     });
}
