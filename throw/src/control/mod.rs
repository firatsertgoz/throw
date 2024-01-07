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
