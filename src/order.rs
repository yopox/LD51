use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::data::{Ingredient, Menu};
use crate::Labels;

pub struct Order {
    pub ingredients: Vec<Ingredient>,
}

pub struct OrderPlugin;

/// Event sent to request a new order
pub struct NewOrderEvent;

/// Event sent when the player has finished a burger
pub struct BurgerFinishedEvent(pub Vec<Ingredient>);

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Menu::Uno)
            .insert_resource(Order {
                ingredients: vec![],
            })
            .add_event::<NewOrderEvent>()
            .add_event::<BurgerFinishedEvent>()
            .add_system(add_order.label(Labels::Logic))
            .add_system(receive_burger);
    }
}

fn generate_order(menu: Menu) -> Order {
    let ingredients = menu.ingredients();
    let mut rng = thread_rng();
    let nb_dist = rand::distributions::Uniform::new(2, ingredients.len());
    let nb = rng.sample(nb_dist);
    return Order { ingredients: ingredients.choose_multiple(&mut rng, nb).cloned().collect() };
}

fn add_order(
    mut commands: Commands,
    menu: Res<Menu>,
    mut order: ResMut<Order>,
    mut new_order_reader: EventReader<NewOrderEvent>
) {
    for _ in new_order_reader.iter() {
        order.ingredients = generate_order(*menu).ingredients;
        println!("Spawned a new order.");
    }
}

fn receive_burger(
    mut ev_burger_sent: EventReader<BurgerFinishedEvent>,
    mut ev_new_order: EventWriter<NewOrderEvent>,
) {
    for BurgerFinishedEvent(ingredients) in ev_burger_sent.iter() {
        // TODO: Compare ingredients with the current order and update score
        ev_new_order.send(NewOrderEvent);
        return;
    }
}
