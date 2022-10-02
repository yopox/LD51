use std::ops::Add;
use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, Delay, EaseMethod, Tracks, Tween, TweenCompleted, TweeningType};

use crate::cooking::CurrentBurger;
use crate::loading::TextureAssets;
use crate::order::{BurgerFinishedEvent, Order};
use crate::restaurant::ShowOrderEvent;
use crate::score::{EXTRA_TIME_PER_BURGER, Score, TIME_PER_INGREDIENT};
use crate::tween::{EV_CUSTOMER_EXITED, EV_CUSTOMER_WAITING_TIME_ELAPSED};
use crate::{tween, DummyComponent, GameState, Labels};

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
        app.add_system_set(
            SystemSet::on_update(GameState::Cooking)
                .label(Labels::UI)
                .after(Labels::LogicReceiver)
                .with_system(update_customers)
                .with_system(customer_enter)
                .with_system(customer_exit)
                .with_system(watch_customer_exited)
                .with_system(watch_customer_waiting_time),
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
                )
                .with_completed_event(EV_CUSTOMER_WAITING_TIME_ELAPSED),
            ])));
    }
}

fn customer_pos() -> Vec3 {
    Vec3::new(248., 40., 2.)
}

fn customer_enter(
    mut commands: Commands,
    mut ev_call_new_customer: EventReader<CallNewCustomer>,
    textures: Res<TextureAssets>,
) {
    for CallNewCustomer in ev_call_new_customer.iter() {
        // Spawn a new customer
        let customer_pos = customer_pos();
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
                tween::fake_tween(tween::TWEEN_TIME)
                    .with_completed_event(tween::EV_CUSTOMER_ARRIVED),
            ))
            .insert(Animator::new(
                Delay::new(Duration::from_millis(tween::TWEEN_TIME))
                    .then(tween::tween_opacity(tween::TWEEN_TIME, true)),
            ))
            .insert(Animator::new(
                Delay::new(Duration::from_millis(tween::TWEEN_TIME)).then(tween::tween_position(
                    customer_pos.xy().clone().add(Vec2::new(32., 0.)),
                    customer_pos.xy().clone(),
                    customer_pos.z,
                    tween::TWEEN_TIME,
                )),
            ))
            .insert(DummyComponent)
            .insert(CurrentCustomer)
            .insert(CustomerUI);
    }
}

fn customer_exit(
    mut commands: Commands,
    mut ev_burger: EventReader<BurgerFinishedEvent>,
    customer: Query<Entity, With<CurrentCustomer>>,
) {
    for BurgerFinishedEvent(_, _) in ev_burger.iter() {
        if let Ok(current_customer) = customer.get_single() {
            let customer_pos = customer_pos();
            commands
                .entity(current_customer)
                .insert(Animator::new(
                    tween::tween_opacity(tween::TWEEN_TIME, false)
                        .with_completed_event(tween::EV_DELETE)
                        .with_completed_event(tween::EV_CUSTOMER_EXITED),
                ))
                .insert(Animator::new(tween::tween_position(
                    customer_pos.xy(),
                    customer_pos.xy() + Vec2::new(32., 0.),
                    customer_pos.z,
                    tween::TWEEN_TIME,
                )))
                .remove::<CurrentCustomer>();
        }
    }
}

fn watch_customer_exited(
    score: Res<Score>,
    mut state: ResMut<State<GameState>>,
    mut ev_tween_finished: EventReader<TweenCompleted>,
    mut ev_call_new_customer: EventWriter<CallNewCustomer>,
) {
    for ev in ev_tween_finished.iter() {
        if ev.user_data == EV_CUSTOMER_EXITED {
            if score.lives > 0 {
                ev_call_new_customer.send(CallNewCustomer);
            } else {
                state.set(GameState::GameOver).unwrap_or_default();
            }
        }
    }
}

fn watch_customer_waiting_time(
    current_burger: Res<CurrentBurger>,
    mut ev_tween_finished: EventReader<TweenCompleted>,
    mut ev_burger_completed: EventWriter<BurgerFinishedEvent>,
) {
    for ev in ev_tween_finished.iter() {
        if ev.user_data == EV_CUSTOMER_WAITING_TIME_ELAPSED {
            ev_burger_completed.send(BurgerFinishedEvent(false, current_burger.ingredients.len()))
        }
    }
}

fn clean_customers(mut commands: Commands, spawned_ui_entities: Query<Entity, With<CustomerUI>>) {
    for e in spawned_ui_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
