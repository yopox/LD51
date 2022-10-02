use std::ops::Add;
use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy_tweening::{Animator, Delay, EaseMethod, Tracks, Tween, TweeningType};
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};

use crate::{DummyComponent, GameState, Labels, tween};
use crate::loading::TextureAssets;
use crate::order::Order;
use crate::restaurant::ShowOrderEvent;
use crate::score::{EXTRA_TIME_PER_BURGER, TIME_PER_INGREDIENT};

pub struct CustomerPlugin;

#[derive(Component)]
struct CustomerUI;

#[derive(Component)]
struct CurrentCustomer;

#[derive(Component)]
struct CustomerTimer;

pub struct CallNewCustomer;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .label(Labels::UI)
                .after(Labels::LogicReceiver)
                .with_system(update_customers)
                .with_system(next_customer)
            )
            .add_system_set(SystemSet::on_exit(GameState::Cooking).with_system(clean_customers))
            .add_event::<CallNewCustomer>();
    }
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

fn next_customer(
    mut commands: Commands,
    mut ev_burger: EventReader<CallNewCustomer>,
    customer: Query<Entity, With<CurrentCustomer>>,
    textures: Res<TextureAssets>,
) {
    let customer_pos = Vec3::new(248., 40., 2.);

    for CallNewCustomer in ev_burger.iter() {
        if let Ok(current_customer) = customer.get_single() {
            commands
                .entity(current_customer)
                .insert(Animator::new(
                    tween::tween_opacity(tween::TWEEN_TIME, false)
                        .with_completed_event(tween::EV_DELETE)
                ))
                .insert(Animator::new(
                    tween::tween_position(
                        customer_pos.xy().clone(),
                        customer_pos.xy().clone().add(Vec2::new(32., 0.)),
                        customer_pos.z,
                        tween::TWEEN_TIME
                    )
                ))
                .remove::<CurrentCustomer>();
        }

        // Spawn a new customer
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.characters.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: Anchor::BottomLeft,
                    color: Color::rgba(1., 1., 1., 0.),
                    ..Default::default()
                },
                transform: Transform::from_translation(customer_pos),
                ..Default::default()
            })
            .insert(Animator::new(
                Delay::new(Duration::from_millis(tween::TWEEN_TIME))
                    .then(tween::fake_tween().with_completed_event(tween::EV_CUSTOMER_ARRIVED))
            ))
            .insert(Animator::new(
                Delay::new(Duration::from_millis(tween::TWEEN_TIME))
                    .then(tween::tween_opacity(tween::TWEEN_TIME, true))
            ))
            .insert(Animator::new(
                Delay::new(Duration::from_millis(tween::TWEEN_TIME))
                    .then(tween::tween_position(
                        customer_pos.xy().clone().add(Vec2::new(32., 0.)),
                        customer_pos.xy().clone(),
                        customer_pos.z,
                        tween::TWEEN_TIME
                    ))
            ))
            .insert(DummyComponent)
            .insert(CurrentCustomer)
            .insert(CustomerUI);
        break;
    }
    ev_burger.clear();
}

fn clean_customers(mut commands: Commands, spawned_ui_entities: Query<Entity, With<CustomerUI>>) {
    for e in spawned_ui_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
