use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    AppLoading,
    AppRunning,

    // FIXME: not sure
    LoadingGame,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    None,
    InGame,
    InLoading,
}

// tag components for all entities within a certain state (for despawning them if needed) , FIXME: seems kinda hack-ish
#[derive(Component)]
pub struct InCoreLoading;
#[derive(Component)]
pub struct InAppLoading;
#[derive(Component)]
pub struct InAppRunning;

#[derive(Component, Default)]
pub struct InMenu;
#[derive(Component, Default)]
pub struct InGame;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>().add_state::<GameState>();
    }
}
