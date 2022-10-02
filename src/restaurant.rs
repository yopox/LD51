use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::{GameState, Labels};
use crate::button::{Letter, spawn_button};
use crate::cooking::CurrentBurger;
use crate::ingredients::{Ingredient, Menu};
use crate::loading::{FontAssets, TextureAssets};
use crate::order::{MenuOnDisplay, Order};

pub struct RestaurantPlugin;

impl Plugin for RestaurantPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Cooking)
                .with_system(init_restaurant)
                .with_system(init_menu),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Cooking)
                .label(Labels::UI)
                .after(Labels::LogicSender)
                .after(Labels::LogicReceiver)
                .with_system(update_arrow)
                .with_system(show_order)
                .with_system(show_menu),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Cooking)
                .with_system(add_ingredient_watcher)
                .with_system(add_ingredient_to_menu),
        )
        .add_system_set(SystemSet::on_exit(GameState::Cooking).with_system(clean_restaurant))
        .insert_resource(AddIngredientTimer(Timer::new(
            Duration::from_secs(10),
            true,
        )))
        .add_event::<ShowOrderEvent>()
        .add_event::<AddIngredientEvent>();
    }
}

pub struct ShowOrderEvent;

#[derive(Component)]
struct CurrentOrderIngredient;

#[derive(Component)]
struct Arrow;

#[derive(Component)]
struct RestaurantUi;

fn init_restaurant(mut commands: Commands, textures: Res<TextureAssets>, fonts: Res<FontAssets>) {
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
            transform: Transform {
                translation: Vec3::new(24., 176., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
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
                        index: order.ingredients.get(i).unwrap().atlas_key(i == 0),
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(192., 64. + 8. * i as f32, 2. + i as f32 / 20.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
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

struct AddIngredientEvent(pub Ingredient);

static MENU_SIZE: usize = 8;

fn add_ingredient_watcher(
    time: Res<Time>,
    menu: Res<Menu>,
    menu_on_display: Res<MenuOnDisplay>,
    mut timer: ResMut<AddIngredientTimer>,
    mut ev_add_ingredient: EventWriter<AddIngredientEvent>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        menu.ingredients().shuffle(&mut thread_rng());
        for ingredient in menu.ingredients() {
            if menu_on_display.ingredients.len() < MENU_SIZE && !menu_on_display.ingredients.contains(&ingredient) {
                ev_add_ingredient.send(AddIngredientEvent(ingredient));
                return;
            }
        }
    }
}

fn add_ingredient_to_menu(
    mut menu: ResMut<MenuOnDisplay>,
    mut ev_add_ingredient: EventReader<AddIngredientEvent>,
) {
    for &AddIngredientEvent(ingredient) in ev_add_ingredient.iter() {
        menu.ingredients.push(ingredient);
    }
}

#[derive(Component)]
struct CurrentMenuIngredient(Ingredient);

fn spawn_menu_item(
    ingredient: Ingredient,
    item_number: u8,
    mut commands: &mut Commands,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
) {
    let button_pos = Vec2::new(24., 145. - 16. * item_number as f32);
    spawn_button(
        &mut commands,
        button_pos,
        ingredient.key(),
        &textures,
        &fonts,
    );

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: ingredient.name(),
                    style: TextStyle {
                        font: fonts.axg.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(44., 158. - 16. * item_number as f32, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CurrentMenuIngredient(ingredient))
        .insert(RestaurantUi);
}

fn init_menu(
    menu: Res<MenuOnDisplay>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
) {
    spawn_menu_item(Ingredient::Bread, 0, &mut commands, &textures, &fonts);
    for (i, &ingredient) in menu.ingredients.iter().enumerate() {
        spawn_menu_item(ingredient, 1 + i as u8, &mut commands, &textures, &fonts);
    }
}

fn show_menu(
    mut ev_add_ingredient: EventReader<AddIngredientEvent>,
    mut commands: Commands,
    current_ingredients: Query<Entity, With<CurrentMenuIngredient>>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
) {
    for &AddIngredientEvent(ingredient) in ev_add_ingredient.iter() {
        spawn_menu_item(
            ingredient,
            current_ingredients.iter().count() as u8,
            &mut commands,
            &textures,
            &fonts,
        );
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
