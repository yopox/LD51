use bevy::prelude::*;

use crate::data::Ingredient;
use crate::GameState;
use crate::input::KeyboardEvent;
use crate::loading::TextureAssets;

pub struct CookingPlugin;

impl Plugin for CookingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentBurger>()
            .add_system_set(SystemSet::on_enter(GameState::Cooking)
                .with_system(reset_order)
            )
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .with_system(add_ingredient)
            );
    }
}

#[derive(Default)]
struct CurrentBurger {
    ingredients: Vec<Ingredient>,
}

#[derive(Component)]
struct CurrentBurgerIngredient;

fn reset_order(
    mut order: ResMut<CurrentBurger>,
) {
    order.ingredients = vec![];
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
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.ingredients.clone(),
                    sprite: TextureAtlasSprite {
                        index: ingredient.atlas_key(),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0., -48. + 8. * current_burger.ingredients.len() as f32, 0.),
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