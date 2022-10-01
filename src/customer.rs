use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::loading::TextureAssets;

pub struct CustomerPlugin;

#[derive(Component)]
struct CustomerUI;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Cooking)
                    .with_system(init_customers),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .with_system(update_customers),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Cooking)
                    .with_system(clean_customers),
            );
    }
}

fn init_customers(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.characters.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(248., 40., 2.),
                ..Default::default()
            },
            ..Default::default()
        }).insert(CustomerUI);
}

fn update_customers(

) {

}

fn clean_customers(
    mut commands: Commands,
    spawned_ui_entities: Query<Entity, With<CustomerUI>>
) {
    for e in spawned_ui_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}