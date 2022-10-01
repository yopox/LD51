use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

fn setup_menu(
    mut commands: Commands,
) {
}

fn cleanup_menu(
    mut commands: Commands,
    button: Query<Entity, With<Button>>
) {
}
