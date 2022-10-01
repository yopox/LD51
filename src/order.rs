use std::sync::Arc;
use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::data::{Ingredient, Menu};

#[derive(Component)]
pub struct Order {
    pub ingredients: Vec<Ingredient>,
}

pub struct OrderPlugin;

pub struct NewOrderEvent;

pub struct NewOrderTimer {
    timer: Timer,
}

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Menu::Uno)
            .insert_resource(NewOrderTimer {
                timer: Timer::new(Duration::from_secs(10), true),
            })
            .add_event::<NewOrderEvent>()
            .add_system(add_order)
            .add_system(generate_order_every_ten_seconds);
    }
}

fn generate_order(menu: Menu) -> Order {
    let ingredients = menu.ingredients();
    let mut rng = thread_rng();
    let nb_dist = rand::distributions::Uniform::new(2, ingredients.len());
    let nb = rng.sample(nb_dist);
    return Order { ingredients: ingredients.choose_multiple(&mut rng, nb).cloned().collect() };
}

fn add_order(mut commands: Commands, menu: Res<Menu>, mut new_order_reader: EventReader<NewOrderEvent>) {
    for _ in new_order_reader.iter() {
        commands.spawn_bundle((generate_order(*menu),));
        println!("Spawned a new order.");
    }
}

fn generate_order_every_ten_seconds(
    mut ev_new_order: EventWriter<NewOrderEvent>,
    time: Res<Time>,
    mut new_order_timer: ResMut<NewOrderTimer>
) {
    // tick the timer
    new_order_timer.timer.tick(time.delta());

    if new_order_timer.timer.finished() {
        ev_new_order.send(NewOrderEvent)
    }
}