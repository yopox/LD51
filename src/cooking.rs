use std::ops::Add;
use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_tweening::{Animator, Delay, Sequence};

use crate::{GameState, Labels, tween};
use crate::audio::{BGM, PlayBgmEvent, PlaySfxEvent, SFX};
use crate::customer::CallNewCustomer;
use crate::ingredients::Ingredient;
use crate::input::KeyboardEvent;
use crate::loading::{FontAssets, TextureAssets};
use crate::order::{BurgerFinishedEvent, MenuOnDisplay, Order};
use crate::score::Score;
use crate::tween::{tween_position, tween_text_opacity};

pub struct CookingPlugin;

impl Plugin for CookingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBurger>()
            .insert_resource(ExpectingOrder(false))
            .insert_resource(MadnessMode(false))
            .add_system_set(
                SystemSet::on_enter(GameState::Cooking)
                    .before(Labels::LogicSender)
                    .with_system(start_cooking),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .before(Labels::LogicSender)
                    .with_system(delete_current),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .label(Labels::LogicSender)
                    .before(Labels::LogicReceiver)
                    .with_system(send_order),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .label(Labels::UI)
                    .after(Labels::LogicReceiver)
                    .with_system(add_ingredient)
                    .with_system(display_streak_or_miss)
                    .with_system(animate_burger),
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

pub struct MadnessMode(pub bool);

fn start_cooking(
    mut ev_call_customer: EventWriter<CallNewCustomer>,
    mut bgm: EventWriter<PlayBgmEvent>,
    is_madness: Res<MadnessMode>,
) {
    // Call the first customer
    ev_call_customer.send(CallNewCustomer);
    bgm.send(PlayBgmEvent(if is_madness.0 { BGM::Madness } else { BGM::Classic }));
}

fn add_ingredient(
    mut input: EventReader<KeyboardEvent>,
    mut current_burger: ResMut<CurrentBurger>,
    mut ev_sfx: EventWriter<PlaySfxEvent>,
    menu: Res<MenuOnDisplay>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for KeyboardEvent(key) in input.iter() {
        if let Some(ingredient) = Ingredient::from_key(&key) {
            // Check that the ingredient has been in the menu
            if !menu.ingredients_seen.contains(&ingredient) {
                continue;
            }

            // Play a sound
            ev_sfx.send(PlaySfxEvent(ingredient.sfx()));

            // Display the added ingredient
            let ingredients_nb = current_burger.ingredients.len();
            let ingredient_pos_starting = Vec2::new(
                116. + if ingredients_nb % 2 == 0 { -4. } else { 4. },
                14. + 8. * ingredients_nb as f32,
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
                    transform: Transform::from_translation(
                        ingredient_pos_starting.extend(ingredient_z),
                    ),
                    ..Default::default()
                })
                .insert(Animator::new(tween::tween_opacity(
                    tween::TWEEN_TIME / 2,
                    true,
                )))
                .insert(Animator::new(tween::tween_position(
                    ingredient_pos_starting,
                    ingredient_pos,
                    ingredient_z,
                    tween::TWEEN_TIME,
                )))
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
                            .with_completed_event(tween::EV_DELETE),
                    ))
                    .insert(Animator::new(tween::tween_position(
                        transform.translation.xy(),
                        transform.translation.xy().add(Vec2::new(8., 0.)),
                        transform.translation.z,
                        tween::TWEEN_TIME,
                    )))
                    .remove::<CurrentBurgerIngredient>();
            }
            current_burger.ingredients.clear();
        }
    }
}

fn send_order(
    order: Res<Order>,
    expecting_order: Res<ExpectingOrder>,
    current_burger: Res<CurrentBurger>,
    mut input: EventReader<KeyboardEvent>,
    mut ev_send_burger: EventWriter<BurgerFinishedEvent>,
    mut ev_sfx: EventWriter<PlaySfxEvent>,
    mut commands: Commands,
) {
    for KeyboardEvent(char) in input.iter() {
        if *char == ' ' {
            if !expecting_order.0 {
                return;
            }

            if current_burger.ingredients.len() > 0 {
                commands.insert_resource(ExpectingOrder(false));
                let correct = current_burger.ingredients == order.ingredients;
                ev_sfx.send(PlaySfxEvent(if correct { SFX::CorrectOrder } else { SFX::IncorrectOrder }));
                ev_send_burger.send(BurgerFinishedEvent(
                    correct,
                    current_burger.ingredients.len(),
                ));
            } else {
                // TODO: Visual error "can't send an empty order"
            }
        }
    }
}

fn animate_burger(
    mut commands: Commands,
    mut ev_burger_finished: EventReader<BurgerFinishedEvent>,
    mut current_burger: ResMut<CurrentBurger>,
    ingredients: Query<(Entity, &Transform), With<CurrentBurgerIngredient>>,
) {
    for BurgerFinishedEvent(success, _) in ev_burger_finished.iter() {
        for (entity, transform) in ingredients.iter() {
            let ingredient_position = transform.translation.xy();
            commands
                .entity(entity)
                .insert(Animator::new(
                    Delay::new(Duration::from_millis(tween::TWEEN_TIME / 6)).then(
                        tween::tween_opacity(tween::TWEEN_TIME, false)
                            .with_completed_event(tween::EV_DELETE),
                    ),
                ))
                .insert(Animator::new(match success {
                    true => win_sequence(ingredient_position, transform.translation.z),
                    false => lose_sequence(ingredient_position, transform.translation.z),
                }))
                .remove::<CurrentBurgerIngredient>();
        }
        current_burger.ingredients.clear();
        break;
    }
    ev_burger_finished.clear();
}

fn win_sequence(position: Vec2, z: f32) -> Sequence<Transform> {
    Sequence::new([
        tween::tween_position(
            position.clone(),
            position.clone().add(Vec2::new(0., -1.)),
            z,
            tween::TWEEN_TIME / 6,
        ),
        tween::tween_position(
            position.clone().add(Vec2::new(0., -1.)),
            position.clone().add(Vec2::new(0., 4.)),
            z,
            tween::TWEEN_TIME,
        ),
    ])
}

fn lose_sequence(position: Vec2, z: f32) -> Sequence<Transform> {
    let amplitude = 3.;
    let time_factor = 6;

    let mut seq = vec![tween::tween_position(
        position.clone(),
        position.clone().add(Vec2::new(-1. * amplitude, 0.)),
        z,
        tween::TWEEN_TIME / time_factor,
    )];

    for i in 0..6 {
        let modif = if i % 2 == 0 { 1. } else { -1. };
        seq.push(tween::tween_position(
            position
                .clone()
                .add(Vec2::new(-1. * amplitude * modif as f32, 0.)),
            position
                .clone()
                .add(Vec2::new(amplitude * modif as f32, 0.)),
            z,
            tween::TWEEN_TIME / time_factor,
        ))
    }
    Sequence::new(seq)
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

        let starting_position = Vec3::new(140., 32. + 8. * nb_ingredients as f32, 1.);

        commands
            .spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text,
                        style: TextStyle {
                            font: fonts.axg.clone(),
                            font_size: 24.0,
                            color: Color::rgba(1., 1., 1., 0.),
                        },
                    }],
                    alignment: TextAlignment::CENTER,
                },
                transform: Transform::from_translation(starting_position),
                ..Default::default()
            })
            .insert(CookingUI)
            .insert(Animator::new(
                tween_text_opacity(Color::WHITE, 1500, false)
            ))
            .insert(Animator::new(
                tween_position(starting_position.xy(), starting_position.xy() + Vec2::new(0., 12.), 10., 1500)
            ));
    }
}

fn clean_cooking_ui(
    mut commands: Commands,
    spawned_ui_components: Query<Entity, With<CookingUI>>,
    mut order: ResMut<CurrentBurger>,
) {
    for e in spawned_ui_components.iter() {
        commands.entity(e).despawn_recursive();
    }
    order.ingredients = vec![];
}
