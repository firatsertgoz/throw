use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Level;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup_graphics);
        // app.add_systems(Startup, setup_physics);
    }
}
