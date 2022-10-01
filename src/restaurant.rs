use bevy::prelude::*;
use bevy::sprite::Anchor;

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

#[derive(Component)]
struct RestaurantUi;

fn init_restaurant(
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.restaurant.clone(),
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RestaurantUi);

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.bubble.clone(),
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(184., 64., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RestaurantUi);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(242., 0., 2.),
                ..Default::default()
            },
            texture: textures.arrow.clone(),
            ..Default::default()
        })
        .insert(Arrow)
        .insert(RestaurantUi);
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
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(192., 72. + 8. * i as f32, 2.),
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
    transform.translation.y = 72. + 8. * current_burger.ingredients.len() as f32;
    visibility.is_visible = current_burger.ingredients.len() < order.ingredients.len();
}