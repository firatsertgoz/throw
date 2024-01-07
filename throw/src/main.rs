mod assets;
mod camera;
mod control;
mod lighting;
mod physics;
mod player;
mod state;

use assets::{AssetsPlugin, GameAssets};
use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_editor_pls::prelude::*;
use bevy_gltf_blueprints::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use camera::CameraPlugin;
use control::BodyPlugin;
use lighting::LightingPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use state::{AppState, GameState, StatePlugin};
use std::path::PathBuf;

use crate::state::InAppRunning;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlueprintsPlugin {
            library_folder: PathBuf::from("library"),
            aabbs: true,
            ..default()
        })
        .add_plugins(AssetsPlugin)
        .add_plugins(StatePlugin)
        //.add_systems(Startup, setup_graphics)
        .add_systems(OnEnter(AppState::AppRunning), setup_game)
        .add_systems(
            Update,
            spawn_blueprint.run_if(in_state(AppState::AppRunning)),
        )
        .add_systems(Startup, setup_physics)
        .add_plugins(LightingPlugin)
        .add_plugins(EditorPlugin::default())
        .add_plugins(BodyPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .run();
}

// fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.insert_resource(MyGltf(asset_server.load("Scene.glb")));
// }

#[derive(Debug, Component)]
pub struct Ground;

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn((Collider::cuboid(100.0, 0.1, 100.0), Ground))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

#[derive(Resource)]
pub struct MyGltf(pub Handle<Gltf>);

pub fn setup_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    models: Res<Assets<bevy::gltf::Gltf>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    println!("setting up all stuff");
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    // here we actually spawn our game world/level
    commands.spawn((
        SceneBundle {
            // note: because of this issue https://github.com/bevyengine/bevy/issues/10436, "world" is now a gltf file instead of a scene
            scene: models
                .get(game_assets.world.id())
                .expect("main level should have been loaded")
                .scenes[0]
                .clone(),
            ..default()
        },
        bevy::prelude::Name::from("world"),
        GameWorldTag,
        InAppRunning,
    ));

    next_game_state.set(GameState::InGame)
}

fn spawn_blueprint(
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    mut game_world: Query<(Entity, &Children), With<GameWorldTag>>,
) {
    // let world = game_world.get_single_mut().unwrap();
    // let world = world.1[0];
    if keycode.just_pressed(KeyCode::F) {
        let new_entity = commands
            .spawn((
                BluePrintBundle {
                    blueprint: BlueprintName("Player".to_string()),
                    transform: TransformBundle::from_transform(Transform::from_xyz(0.0, 2.0, 0.0)),
                    ..Default::default()
                },
                bevy::prelude::Name::from(format!("test{}", "monsta")),
                // BlueprintName("Health_Pickup".to_string()),
                // SpawnHere,
                // TransformBundle::from_transform(Transform::from_xyz(x, 2.0, y)),
                Velocity {
                    linvel: Vec3::new(0.0, 0.0, 0.0),
                    angvel: Vec3::new(0.0, 0.0, 0.0),
                },
            ))
            .id();
        //  commands.entity(world).add_child(new_entity);
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// helper marker component
pub struct LoadedMarker;
