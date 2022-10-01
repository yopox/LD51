use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{GameState, Labels};
use crate::data::{Ingredient, Menu};
use crate::restaurant::ShowOrderEvent;
use crate::score::Score;

#[derive(Default)]
pub struct Order {
    pub ingredients: Vec<Ingredient>,
    pub creation_time: Duration,
}

pub struct OrderPlugin;

/// Event sent to request a new order
pub struct NewOrderEvent;

/// Event sent when the player has finished a burger
pub struct BurgerFinishedEvent(pub Vec<Ingredient>);

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Menu::Uno)
            .init_resource::<Order>()
            .add_event::<NewOrderEvent>()
            .add_event::<BurgerFinishedEvent>()
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .label(Labels::Logic)
                .with_system(add_order)
                .with_system(receive_burger)
            );
    }
}

fn generate_order(menu: Menu) -> Vec<Ingredient> {
    let ingredients = menu.ingredients();
    let mut rng = thread_rng();
    let nb_dist = rand::distributions::Uniform::new(2, ingredients.len());
    let nb = rng.sample(nb_dist);
    let mut recipe = vec![Ingredient::Bread];
    ingredients
        .choose_multiple(&mut rng, nb).cloned().collect::<Vec<Ingredient>>().iter()
        .for_each(|x| recipe.push(*x));
    recipe.push(Ingredient::Bread);
    return recipe;
}

fn add_order(
    menu: Res<Menu>,
    time: Res<Time>,
    mut order: ResMut<Order>,
    mut new_order_reader: EventReader<NewOrderEvent>,
    mut ev_show_order: EventWriter<ShowOrderEvent>,
) {
    for _ in new_order_reader.iter() {
        order.ingredients = generate_order(*menu);
        order.creation_time = time.time_since_startup();
        ev_show_order.send(ShowOrderEvent);
    }
}

fn receive_burger(
    time: Res<Time>,
    order: Res<Order>,
    mut score: ResMut<Score>,
    mut ev_burger_sent: EventReader<BurgerFinishedEvent>,
    mut ev_new_order: EventWriter<NewOrderEvent>,
) {
    for BurgerFinishedEvent(ingredients) in ev_burger_sent.iter() {
        if *ingredients == order.ingredients {
            let duration = time.time_since_startup() - order.creation_time;
            score.compute_on_success(duration.as_secs_f64(), ingredients.len());
            ev_new_order.send(NewOrderEvent);
        } else {
            // Do not update order
            score.compute_on_failure()
        }

        return;
    }
}
