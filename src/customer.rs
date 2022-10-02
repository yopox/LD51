use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy_tweening::{Animator, EaseFunction, EaseMethod, Tracks, Tween, TweeningType};
use bevy_tweening::lens::{TextColorLens, TransformPositionLens, TransformScaleLens};

use crate::GameState;
use crate::loading::TextureAssets;
use crate::order::Order;
use crate::restaurant::ShowOrderEvent;
use crate::score::{EXTRA_TIME_PER_BURGER, TIME_PER_INGREDIENT};

pub struct CustomerPlugin;

#[derive(Component)]
struct CustomerUI;

#[derive(Component)]
struct CustomerTimer;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Cooking).with_system(init_customers))
            .add_system_set(SystemSet::on_update(GameState::Cooking).with_system(update_customers))
            .add_system_set(SystemSet::on_exit(GameState::Cooking).with_system(clean_customers));
    }
}

fn init_customers(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.characters.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(248., 40., 2.)),
            ..Default::default()
        })
        .insert(CustomerUI);
}

fn update_customers(
    order: Res<Order>,
    mut commands: Commands,
    mut ev_show_order: EventReader<ShowOrderEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<CustomerTimer>>,
) {
    // Create customer timers
    for _ in ev_show_order.iter() {
        for e in query.iter() {
            commands.entity(e).despawn_recursive()
        }

        let duration = Duration::from_secs_f64(
            EXTRA_TIME_PER_BURGER + order.ingredients.len() as f64 * TIME_PER_INGREDIENT,
        );

        let start_position = Vec3::new(260., 112., 2.);
        let x_size = 50.;

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(1., 1.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(start_position),
                ..default()
            })
            .insert(CustomerUI)
            .insert(CustomerTimer)
            .insert(Animator::new(Tracks::new(vec![
                Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    duration,
                    TransformScaleLens {
                        start: Vec3::new(1., 1., 1.),
                        end: Vec3::new(x_size, 1., 1.),
                    },
                ),
                Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    duration,
                    TransformPositionLens {
                        start: start_position,
                        end: start_position + Vec3::new(x_size / 2., 1., 1.),
                    },
                ),
            ])));
    }
}

fn clean_customers(mut commands: Commands, spawned_ui_entities: Query<Entity, With<CustomerUI>>) {
    for e in spawned_ui_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
