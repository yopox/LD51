use bevy::prelude::*;

use crate::{GameState, Labels};
use crate::cooking::CurrentBurger;
use crate::loading::TextureAssets;
use crate::order::Order;

pub struct RestaurantPlugin;

impl Plugin for RestaurantPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Cooking)
                .with_system(init_restaurant)
            )
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .label(Labels::UI)
                .after(Labels::Logic)
                .with_system(update_arrow)
                .with_system(show_order)
            )
            .add_event::<ShowOrderEvent>();
    }
}

pub struct ShowOrderEvent;

#[derive(Component)]
struct CurrentOrderIngredient;

#[derive(Component)]
struct Arrow;

fn init_restaurant(
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-16., -48., 0.),
                ..Default::default()
            },
            texture: textures.arrow.clone(),
            ..Default::default()
        })
        .insert(Arrow);
}

fn show_order(
    mut ev_show_order: EventReader<ShowOrderEvent>,
    order: Res<Order>,
    current_ingredients: Query<Entity, With<CurrentOrderIngredient>>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for _ in ev_show_order.iter() {
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
                        translation: Vec3::new(-48., -48. + 8. * i as f32, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CurrentOrderIngredient);
        }
    }
}

fn update_arrow(
    current_burger: Res<CurrentBurger>,
    order: Res<Order>,
    mut arrow: Query<(&mut Transform, &mut Visibility), With<Arrow>>,
) {
    let (mut transform, mut visibility) = arrow.single_mut();
    transform.translation.y = -48. + 8. * current_burger.ingredients.len() as f32;
    visibility.is_visible = current_burger.ingredients.len() < order.ingredients.len();
}