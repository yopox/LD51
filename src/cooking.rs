use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{GameState, Labels};
use crate::ingredients::Ingredient;
use crate::input::KeyboardEvent;
use crate::loading::TextureAssets;
use crate::order::{BurgerFinishedEvent, NewOrderEvent};

pub struct CookingPlugin;

impl Plugin for CookingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentBurger>()
            .add_system_set(SystemSet::on_enter(GameState::Cooking)
                .before(Labels::Logic)
                .with_system(start_cooking)
            )
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .with_system(add_ingredient)
                .with_system(delete_current)
                .with_system(send_order)
            );
    }
}

#[derive(Default)]
pub struct CurrentBurger {
    pub ingredients: Vec<Ingredient>,
}

#[derive(Component)]
struct CurrentBurgerIngredient;

fn start_cooking(
    mut order: ResMut<CurrentBurger>,
    mut new_order: EventWriter<NewOrderEvent>,
) {
    // Reset current burger
    order.ingredients = vec![];

    // Request an order
    new_order.send(NewOrderEvent);
}

fn add_ingredient(
    mut input: EventReader<KeyboardEvent>,
    mut current_burger: ResMut<CurrentBurger>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for KeyboardEvent(key) in input.iter() {
        if let Some(ingredient) = Ingredient::from_key(&key) {
            // Display the added ingredient
            let ingredients_nb = current_burger.ingredients.len();
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.ingredients.clone(),
                    sprite: TextureAtlasSprite {
                        index: ingredient.atlas_key(ingredients_nb == 0),
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(124., 22. + 8. * ingredients_nb as f32, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CurrentBurgerIngredient);

            // Add ingredient to the current burger
            current_burger.ingredients.push(ingredient.clone());
        }
    }
}

fn delete_current(
    mut input: EventReader<KeyboardEvent>,
    ingredients: Query<Entity, With<CurrentBurgerIngredient>>,
    mut current_burger: ResMut<CurrentBurger>,
    mut commands: Commands,
) {
    for KeyboardEvent(char) in input.iter() {
        if *char == '<' {
            for entity in ingredients.iter() {
                commands.entity(entity).despawn();
            }
            current_burger.ingredients.clear();
        }
    }
}

fn send_order(
    mut input: EventReader<KeyboardEvent>,
    mut ev_send_burger: EventWriter<BurgerFinishedEvent>,
    ingredients: Query<Entity, With<CurrentBurgerIngredient>>,
    mut current_burger: ResMut<CurrentBurger>,
    mut commands: Commands,
) {
    for KeyboardEvent(char) in input.iter() {
        if *char == '>' {
            if current_burger.ingredients.len() > 0 {
                for entity in ingredients.iter() {
                    commands.entity(entity).despawn();
                }
                ev_send_burger.send(BurgerFinishedEvent(current_burger.ingredients.clone()));
                current_burger.ingredients.clear();
            } else {
                // TODO: Visual error "can't send an empty order"
            }
        }
    }
}