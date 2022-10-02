use std::ops::Add;
use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_tweening::{Animator, Delay, EaseFunction, Sequence, Tween, TweeningType};
use bevy_tweening::lens::{TextColorLens, TransformPositionLens};

use crate::{GameState, Labels, tween};
use crate::customer::CallNewCustomer;
use crate::ingredients::Ingredient;
use crate::input::KeyboardEvent;
use crate::loading::{FontAssets, TextureAssets};
use crate::order::{BurgerFinishedEvent, MenuOnDisplay, Order};
use crate::score::Score;

pub struct CookingPlugin;

impl Plugin for CookingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentBurger>()
            .insert_resource(ExpectingOrder(false))
            .add_system_set(
                SystemSet::on_enter(GameState::Cooking)
                    .before(Labels::LogicSender)
                    .with_system(start_cooking),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .before(Labels::LogicSender)
                    .with_system(delete_current)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .label(Labels::LogicSender)
                    .before(Labels::LogicReceiver)
                    .with_system(send_order)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .label(Labels::UI)
                    .after(Labels::LogicReceiver)
                    .with_system(add_ingredient)
                    .with_system(display_streak_or_miss),
            )
            .add_system_set(SystemSet::on_exit(GameState::Cooking).with_system(clean_cooking_ui));
    }
}

#[derive(Component)]
struct CookingUI;

#[derive(Default)]
pub struct CurrentBurger {
    pub ingredients: Vec<Ingredient>,
}

#[derive(Component)]
struct CurrentBurgerIngredient;

pub struct ExpectingOrder(pub bool);

fn start_cooking(
    mut order: ResMut<CurrentBurger>,
    mut ev_call_customer: EventWriter<CallNewCustomer>,
) {
    // Reset current burger
    order.ingredients = vec![];

    // Call the first customer
    ev_call_customer.send(CallNewCustomer);
}

fn add_ingredient(
    mut input: EventReader<KeyboardEvent>,
    mut current_burger: ResMut<CurrentBurger>,
    menu: Res<MenuOnDisplay>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for KeyboardEvent(key) in input.iter() {
        if let Some(ingredient) = Ingredient::from_key(&key) {
            // Check that the ingredient is in the menu
            if ingredient != Ingredient::Bread && !menu.ingredients.contains(&ingredient) {
                continue;
            }
            // Display the added ingredient
            let ingredients_nb = current_burger.ingredients.len();
            let ingredient_pos_starting = Vec2::new(
                116. + if ingredients_nb % 2 == 0 { -4. } else { 4. },
                14. + 8. * ingredients_nb as f32
            );
            let ingredient_pos = Vec2::new(116., 14. + 8. * ingredients_nb as f32);
            let ingredient_z = 1. + ingredients_nb as f32 / 20.;
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.ingredients.clone(),
                    sprite: TextureAtlasSprite {
                        index: ingredient.atlas_key(ingredients_nb == 0),
                        anchor: Anchor::BottomLeft,
                        color: Color::rgba(1., 1., 1., 0.),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(ingredient_pos_starting.extend(ingredient_z)),
                    ..Default::default()
                })
                .insert(Animator::new(tween::tween_opacity(tween::TWEEN_TIME / 2, true)))
                .insert(Animator::new(tween::tween_position(ingredient_pos_starting, ingredient_pos, ingredient_z, tween::TWEEN_TIME)))
                .insert(CurrentBurgerIngredient)
                .insert(CookingUI);

            // Add ingredient to the current burger
            current_burger.ingredients.push(ingredient.clone());
        }
    }
}

fn delete_current(
    mut input: EventReader<KeyboardEvent>,
    ingredients: Query<(Entity, &Transform), With<CurrentBurgerIngredient>>,
    mut current_burger: ResMut<CurrentBurger>,
    mut commands: Commands,
) {
    for KeyboardEvent(char) in input.iter() {
        if *char == '<' {
            for (entity, transform) in ingredients.iter() {
                commands
                    .entity(entity)
                    .insert(Animator::new(
                        tween::tween_opacity(tween::TWEEN_TIME, false)
                            .with_completed_event(tween::EV_DELETE)
                    ))
                    .insert(Animator::new(
                        tween::tween_position(
                            transform.translation.xy(),
                            transform.translation.xy().add(Vec2::new(8., 0.)),
                            transform.translation.z,
                            tween::TWEEN_TIME,
                        )
                    ))
                    .remove::<CurrentBurgerIngredient>();
            }
            current_burger.ingredients.clear();
        }
    }
}

fn send_order(
    order: Res<Order>,
    expecting_order: Res<ExpectingOrder>,
    mut input: EventReader<KeyboardEvent>,
    mut ev_send_burger: EventWriter<BurgerFinishedEvent>,
    ingredients: Query<(Entity, &Transform), With<CurrentBurgerIngredient>>,
    mut current_burger: ResMut<CurrentBurger>,
    mut commands: Commands,
) {
    for KeyboardEvent(char) in input.iter() {
        if *char == ' ' {
            if !expecting_order.0 { return; }

            if current_burger.ingredients.len() > 0 {
                for (entity, transform) in ingredients.iter() {
                    let ingredient_position = transform.translation.xy();
                    commands
                        .entity(entity)
                        .insert(Animator::new(
                            Delay::new(Duration::from_millis(tween::TWEEN_TIME / 6))
                                .then(tween::tween_opacity(tween::TWEEN_TIME, false)
                                    .with_completed_event(tween::EV_DELETE))
                        ))
                        .insert(Animator::new(
                            Sequence::new([
                                tween::tween_position(
                                    ingredient_position.clone(),
                                    ingredient_position.clone().add(Vec2::new(0., -1.)),
                                    transform.translation.z,
                                    tween::TWEEN_TIME / 6,
                                ),
                                tween::tween_position(
                                    ingredient_position.clone().add(Vec2::new(0., -1.)),
                                    ingredient_position.clone().add(Vec2::new(0., 4.)),
                                    transform.translation.z,
                                    tween::TWEEN_TIME,
                                ),
                            ])
                        ))
                        .remove::<CurrentBurgerIngredient>();
                }
                commands.insert_resource(ExpectingOrder(false));
                ev_send_burger.send(BurgerFinishedEvent(
                    current_burger.ingredients == order.ingredients,
                    current_burger.ingredients.len(),
                ));
                current_burger.ingredients.clear();
            } else {
                // TODO: Visual error "can't send an empty order"
            }
        }
    }
}

fn display_streak_or_miss(
    score: Res<Score>,
    mut ev_send_burger: EventReader<BurgerFinishedEvent>,
    fonts: Res<FontAssets>,
    mut commands: Commands,
) {
    for &BurgerFinishedEvent(correct, nb_ingredients) in ev_send_burger.iter() {
        let text = if correct {
            if score.streak > 1 {
                format!("{} CHAIN", score.streak)
            } else {
                continue;
            }
        } else {
            "MISS".to_string()
        };

        let starting_position = Vec3::new(140., 44. + 8. * nb_ingredients as f32, 1.);

        commands
            .spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text,
                        style: TextStyle {
                            font: fonts.axg.clone(),
                            font_size: 16.0,
                            color: Color::rgba(0., 0., 0., 0.),
                        },
                    }],
                    alignment: TextAlignment::CENTER,
                },
                transform: Transform::from_translation(starting_position),
                ..Default::default()
            })
            .insert(CookingUI)
            .insert(Animator::new(Tween::new(
                EaseFunction::CubicOut,
                TweeningType::Once,
                Duration::from_secs_f32(1.5),
                TextColorLens {
                    start: Color::rgba(0., 0., 0., 1.),
                    end: Color::rgba(0., 0., 0., 0.),
                    section: 0,
                },
            )))
            .insert(Animator::new(Tween::new(
                EaseFunction::CubicOut,
                TweeningType::Once,
                Duration::from_secs_f32(1.5),
                TransformPositionLens {
                    start: starting_position,
                    end: starting_position + Vec3::new(0., 12., 0.),
                },
            )));
    }
}

fn clean_cooking_ui(mut commands: Commands, spawned_ui_components: Query<Entity, With<CookingUI>>) {
    for e in spawned_ui_components.iter() {
        commands.entity(e).despawn_recursive();
    }
}
