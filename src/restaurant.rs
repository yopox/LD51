use std::ops::Add;
use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweenCompleted, TweeningType};
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

use crate::{GameState, Labels, spawn_sprite, tween};
use crate::audio::{PlaySfxEvent, SFX};
use crate::button::{Letter, PreventButtonUpdate, spawn_button};
use crate::cooking::CurrentBurger;
use crate::ingredients::{Ingredient, Menu};
use crate::loading::{FontAssets, TextureAssets};
use crate::order::{BurgerFinishedEvent, MenuOnDisplay, Order};
use crate::tween::{EV_ALLOW_BUTTON_UPDATE, EV_CHALK, EV_DELETE, EV_NOTHING, TransformAtlasSpriteAlphaLens, tween_opacity, tween_text_opacity, TWEEN_TIME};

/// Flow of the restaurant:
/// 1. [`crate::cooking::start_cooking`] -> Sends [`crate::customer::CallNewCustomer`] to call the first customer
/// 2. [`crate::customer::customer_enter`] -> Listens to [`crate::customer::CallNewCustomer`] and make the customer appears
/// 3. [`crate::order::add_order`] -> Listens to [`crate::customer::CallNewCustomer`], generates the order of the customer and sends [`ShowOrderEvent`]
/// 4. [`show_order`] -> Shows the order
/// 5. [`crate::cooking::send_order`] -> The user sends an order and the event [`BurgerFinishedEvent`] is sent
///     - [`crate::cooking::display_streak_or_miss`] -> Listens to [`BurgerFinishedEvent`] and displays GUI
///     - [`crate::cooking::animate_burger`] -> Listens to [`BurgerFinishedEvent`] and animates the burger
///     - [`hide_order`] -> Listens to [`BurgerFinishedEvent`] and hide the current order
///     - [`crate::order::receive_burger`] -> Listens to [`BurgerFinishedEvent`], updates the score
///     - [`crate::customer::customer_exit`] -> Listens to [`BurgerFinishedEvent`], make the customer exit and sends [`TweenCompleted { _, crate::tween::EV_CUSTOMER_EXITED }`] when customer has exited
/// 6. [`crate::customer::watch_customer_exited`] -> Sends [`crate::customer::CallNewCustomer`] or sets State to [`crate::GameState::GameOver`]
pub struct RestaurantPlugin;

impl Plugin for RestaurantPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Cooking)
                .label(Labels::UI)
                .with_system(init_restaurant)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Cooking)
                .label(Labels::UI)
                .after(Labels::LogicSender)
                .after(Labels::LogicReceiver)
                .with_system(update_arrow)
                .with_system(show_order)
                .with_system(hide_order)
                .with_system(add_ingredient_watcher)
                .with_system(add_ingredient_to_menu)
                .with_system(show_menu)
                .with_system(chalk),
        )
        .add_system_set(SystemSet::on_exit(GameState::Cooking).with_system(clean_restaurant))
        .insert_resource(AddIngredientTimer(Timer::new(
            Duration::from_secs(10),
            true,
        )))
        .add_event::<ShowOrderEvent>()
        .add_event::<AddIngredientEvent>()
        .add_event::<ShowIngredientEvent>();
    }
}

pub struct ShowOrderEvent;

pub struct ShowIngredientEvent {
    pub replace: bool,
    pub position: usize,
    pub ingredient: Ingredient,
    pub timer: bool,
}

#[derive(Component)]
struct CurrentOrderIngredient;

#[derive(Component)]
struct Arrow;

#[derive(Component)]
struct RestaurantUi;

fn init_restaurant(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>
) {
    spawn_sprite(&mut commands, textures.background.clone(), Vec3::ZERO).insert(RestaurantUi);
    spawn_sprite(&mut commands, textures.bubble.clone(), Vec3::new(184., 64., 0.5)).insert(RestaurantUi);
    spawn_sprite(&mut commands, textures.menu.clone(), Vec3::new(8., 8., 0.75)).insert(RestaurantUi);
    spawn_sprite(&mut commands, textures.counter.clone(), Vec3::new(0., 0., 0.5)).insert(RestaurantUi);
    spawn_sprite(&mut commands, textures.plate.clone(), Vec3::new(104., 16., 0.75,)).insert(RestaurantUi);
    spawn_sprite(&mut commands, textures.bill.clone(), Vec3::new(320. - 64., 180. - 64., 3.,)).insert(RestaurantUi);
    spawn_sprite(&mut commands, textures.arrow.clone(), Vec3::new(242., 0., 2.)).insert(RestaurantUi)
        .insert(Arrow);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "TODAY'S MENU".to_string(),
                    style: TextStyle {
                        font: fonts.axg.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(24., 176., 1.),
            ..Default::default()
        })
        .insert(RestaurantUi);
}

fn hide_order(
    mut commands: Commands,
    mut ev_burger_finished: EventReader<BurgerFinishedEvent>,
    current_ingredients: Query<(Entity, &Transform), With<CurrentOrderIngredient>>,
) {
    for _ in ev_burger_finished.iter() {
        for (entity, transform) in current_ingredients.iter() {
            let ingredient_position = transform.translation.xy();
            commands
                .entity(entity)
                .insert(Animator::new(
                    tween::tween_opacity(tween::TWEEN_TIME, false)
                        .with_completed_event(tween::EV_DELETE))
                )
                .insert(Animator::new(
                    tween::tween_position(
                        ingredient_position.clone(),
                        ingredient_position.clone().add(Vec2::new(4., 0.)),
                        transform.translation.z,
                        tween::TWEEN_TIME,
                    ))
                )
                .remove::<CurrentOrderIngredient>();
        }
    }
}

fn show_order(
    mut ev_show_order: EventReader<ShowOrderEvent>,
    order: Res<Order>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for _ in ev_show_order.iter() {
        for i in 0..order.ingredients.len() {
            let ingredient_y = 60. + 8. * i as f32;
            let ingredient_z = 2. + i as f32 / 20.;

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.ingredients.clone(),
                    sprite: TextureAtlasSprite {
                        index: order.ingredients.get(i).unwrap().atlas_key(i == 0),
                        anchor: Anchor::BottomLeft,
                        color: Color::rgba(1., 1., 1., 0.),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(192., ingredient_y, ingredient_z),
                    ..Default::default()
                })
                .insert(Animator::new(
                    Delay::new(Duration::from_millis(100 + 50 * i as u64))
                        .then(tween::tween_opacity(tween::TWEEN_TIME, true))
                ))
                .insert(Animator::new(
                    Delay::new(Duration::from_millis(100 + 50 * i as u64))
                        .then(tween::tween_position(Vec2::new(192., ingredient_y),
                                                    Vec2::new(192., ingredient_y + 4.),
                                                    ingredient_z, tween::TWEEN_TIME))
                ))
                .insert(CurrentOrderIngredient)
                .insert(RestaurantUi);
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

#[derive(Component)]
struct AddIngredientTimer(pub Timer);

pub struct AddIngredientEvent {
    pub ingredient: Ingredient,
    pub timer: bool,
}

pub static MENU_SIZE: usize = 8;

fn add_ingredient_watcher(
    time: Res<Time>,
    menu: Res<Menu>,
    menu_on_display: Res<MenuOnDisplay>,
    mut timer: ResMut<AddIngredientTimer>,
    mut ev_add_ingredient: EventWriter<AddIngredientEvent>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let ingredients = menu.ingredients();
        let mut ingredients_not_in_menu: Vec<&Ingredient> = ingredients
            .iter().filter(|&i| !menu_on_display.ingredients.contains(i))
            .collect();
        ingredients_not_in_menu.shuffle(&mut thread_rng());
        ev_add_ingredient.send(AddIngredientEvent {
            ingredient: **ingredients_not_in_menu.first().unwrap(),
            timer: true
        });
    }
}

fn add_ingredient_to_menu(
    mut menu: ResMut<MenuOnDisplay>,
    mut ev_add_ingredient: EventReader<AddIngredientEvent>,
    mut ev_show_ingredient: EventWriter<ShowIngredientEvent>,
) {
    for &AddIngredientEvent { ingredient, timer } in ev_add_ingredient.iter() {
        menu.ingredients_seen.insert(ingredient);
        if menu.ingredients.len() <= MENU_SIZE {
            // Add a new item at the end of the menu
            menu.ingredients.push(ingredient);
            ev_show_ingredient.send(ShowIngredientEvent {
                replace: false,
                position: menu.ingredients.iter().position(|&i| i == ingredient).unwrap(),
                ingredient,
                timer
            });
        } else {
            // Replace a menu item
            let mut to_replace = thread_rng().gen_range(2..MENU_SIZE);
            if !ingredient.is_meat() && menu.ingredients.iter().filter(|i| i.is_meat()).count() < 2 {
                while menu.ingredients.get(to_replace).unwrap().is_meat() {
                    to_replace = thread_rng().gen_range(2..MENU_SIZE);
                }
            }
            menu.ingredients.remove(to_replace);
            menu.ingredients.insert(to_replace, ingredient);
            ev_show_ingredient.send(ShowIngredientEvent {
                replace: true,
                position: to_replace,
                ingredient,
                timer
            })
        }
    }
}

#[derive(Component)]
struct CurrentMenuIngredient(u8);

fn spawn_menu_item(
    ingredient: Ingredient,
    item_number: u8,
    mut commands: &mut Commands,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
    timer: bool,
) {
    let button_pos = Vec2::new(20., 145. - 16. * item_number as f32);
    let (button, button_text) = spawn_button(
        &mut commands,
        button_pos,
        ingredient.key(),
        &textures,
        &fonts,
        true
    );

    let text_appear_animator = |color| Animator::new(
        Delay::new(Duration::from_millis(if timer { TWEEN_TIME * 2 } else { 0 })).then(
            tween_text_opacity(color, TWEEN_TIME * 3, true)
        ));

    commands
        .entity(button)
        .insert(Animator::new(
            Delay::new(Duration::from_millis(if timer { TWEEN_TIME * 2 } else { 0 })).then(
                Tween::new(
                    EaseFunction::CubicOut,
                    TweeningType::Once,
                    Duration::from_millis(1),
                    TransformAtlasSpriteAlphaLens {
                        start: 0.,
                        end: 0.,
                    }
                ).with_completed_event(if timer { EV_CHALK } else { EV_NOTHING }).then(
                    tween_opacity(TWEEN_TIME * 3, true).with_completed_event(EV_ALLOW_BUTTON_UPDATE)
                )
            ))
        )
        .insert(PreventButtonUpdate)
        .insert(CurrentMenuIngredient(item_number));

    commands
        .entity(button_text)
        .insert(text_appear_animator(Color::WHITE));

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: ingredient.name(),
                    style: TextStyle {
                        font: fonts.axg.clone(),
                        font_size: 16.0,
                        color: Color::rgba(1., 1., 1., 0.),
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(40., 158. - 16. * item_number as f32, 1.),
            ..Default::default()
        })
        .insert(text_appear_animator(Color::WHITE))
        .insert(CurrentMenuIngredient(item_number))
        .insert(RestaurantUi);
}

fn chalk(
    mut tween_events: EventReader<TweenCompleted>,
    mut sfx: EventWriter<PlaySfxEvent>,
) {
    for &TweenCompleted { entity, user_data } in tween_events.iter() {
        if user_data == EV_CHALK {
            sfx.send(PlaySfxEvent(SFX::Chalk));
        }
    }
}

fn replace_menu_item(
    ingredient: Ingredient,
    item_number: u8,
    mut commands: &mut Commands,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
    mut queries: &mut ParamSet<(
        Query<(Entity, &CurrentMenuIngredient), With<Text>>,
        Query<(Entity, &Children, &CurrentMenuIngredient), With<Letter>>
    )>
) {
    let query = queries.p0();
    for (e, &CurrentMenuIngredient(i)) in query.iter() {
        if item_number == i {
            commands
                .entity(e)
                .insert(Animator::new(
                    tween_text_opacity(Color::WHITE, TWEEN_TIME * 2, false)
                        .with_completed_event(EV_DELETE)
                ));
        }
    }

    let query = queries.p1();
    for (e, children, &CurrentMenuIngredient(i)) in query.iter() {
        if item_number == i {
            commands
                .entity(e)
                .insert(Animator::new(
                    tween_opacity(TWEEN_TIME * 2, false)
                ))
                .insert(PreventButtonUpdate);

            if let Some(child) = children.get(0) {
                commands
                    .entity(*child)
                    .insert(Animator::new(
                        tween_text_opacity(Color::WHITE, TWEEN_TIME * 2, false)
                    ));
            }
        }
    }
    spawn_menu_item(ingredient, item_number, &mut commands, textures, fonts, true);
}

fn show_menu(
    mut ev_show_ingredient: EventReader<ShowIngredientEvent>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
    mut queries: ParamSet<(
        Query<(Entity, &CurrentMenuIngredient), With<Text>>,
        Query<(Entity, &Children, &CurrentMenuIngredient), With<Letter>>
    )>,
) {
    for &ShowIngredientEvent { replace, position, ingredient, timer } in ev_show_ingredient.iter() {
        if replace {
            replace_menu_item(
                ingredient,
                position as u8,
                &mut commands,
                &textures,
                &fonts,
                &mut queries,
            );
        } else {
            spawn_menu_item(
                ingredient,
                position as u8,
                &mut commands,
                &textures,
                &fonts,
                timer,
            );
        }
    }
}

fn clean_restaurant(
    mut commands: Commands,
    spawned_ui_components: Query<Entity, With<RestaurantUi>>,
    buttons: Query<Entity, With<Letter>>,
) {
    for e in spawned_ui_components.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in buttons.iter() {
        commands.entity(e).despawn_recursive();
    }
}
