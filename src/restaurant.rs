use bevy::prelude::*;

use crate::{GameState, Labels};
use crate::loading::TextureAssets;
use crate::order::{NewOrderEvent, Order};

pub struct RestaurantPlugin;

impl Plugin for RestaurantPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .with_system(show_order.after(Labels::Logic))
            );
    }
}

#[derive(Component)]
struct CurrentOrderIngredient;

fn show_order(
    mut ev_new_order: EventReader<NewOrderEvent>,
    order: Res<Order>,
    current_ingredients: Query<Entity, With<CurrentOrderIngredient>>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for NewOrderEvent in ev_new_order.iter() {
        for entity in current_ingredients.iter() {
            commands.entity(entity).despawn();
        }

        for i in 0..order.ingredients.len() {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.ingredients.clone(),
                    sprite: TextureAtlasSprite {
                        index: order.ingredients.get(i).unwrap().atlas_key(),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(-32., -48. + 8. * i as f32, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CurrentOrderIngredient);
        }
    }
}