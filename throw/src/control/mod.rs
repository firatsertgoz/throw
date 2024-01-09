mod body;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use self::body::{setup_torso, setup_spine};

#[derive(Debug, Component)]
pub struct Body;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_torso);
        app.add_systems(Startup, setup_spine);
    }
}
