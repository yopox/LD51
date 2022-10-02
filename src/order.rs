use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{GameState, Labels};
use crate::cooking::ExpectingOrder;
use crate::customer::CallNewCustomer;
use crate::ingredients::{Ingredient, Menu};
use crate::restaurant::ShowOrderEvent;
use crate::score::{LifeIcon, Score};

#[derive(Default)]
pub struct Order {
    pub ingredients: Vec<Ingredient>,
    pub creation_time: Duration,
}

#[derive(Default)]
pub struct MenuOnDisplay {
    pub ingredients: Vec<Ingredient>,
}

impl From<Menu> for MenuOnDisplay {
    fn from(m: Menu) -> Self {
        MenuOnDisplay {
            ingredients: m.ingredients().choose_multiple(&mut thread_rng(), 2).cloned().collect()
        }
    }
}

pub struct OrderPlugin;

/// Event sent when the player has finished a burger
/// the bool indicates if the burger satisfies the customer demands
/// The int indicates how many ingredients were inside the burger
pub struct BurgerFinishedEvent(pub bool, pub usize);

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        let menu_reference = Menu::Uno;

        app.insert_resource(menu_reference)
            .init_resource::<Order>()
            .init_resource::<MenuOnDisplay>()
            .add_event::<BurgerFinishedEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Cooking)
                .before(Labels::UI)
                .with_system(init_menu)
            )
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                .label(Labels::LogicReceiver)
                .before(Labels::UI)
                .after(Labels::LogicSender)
                .with_system(add_order)
                .with_system(receive_burger)
            );
    }
}
fn init_menu(
    menu: Res<Menu>,
    mut menu_on_display: ResMut<MenuOnDisplay>,
) {
    menu_on_display.ingredients = menu.basic_ingredients();
}

fn add_order(
    menu: Res<MenuOnDisplay>,
    menu_ref: Res<Menu>,
    time: Res<Time>,
    mut commands: Commands,
    mut order: ResMut<Order>,
    mut ev_new_customer: EventReader<CallNewCustomer>,
    mut ev_show_order: EventWriter<ShowOrderEvent>,
) {
    for CallNewCustomer in ev_new_customer.iter() {
        order.ingredients = menu_ref.generate_order(&menu.ingredients);
        order.creation_time = time.time_since_startup();
        commands.insert_resource(ExpectingOrder(true));
        ev_show_order.send(ShowOrderEvent);
        break;
    }
    ev_new_customer.clear();
}

fn receive_burger(
    time: Res<Time>,
    order: Res<Order>,
    mut score: ResMut<Score>,
    mut ev_burger_sent: EventReader<BurgerFinishedEvent>,
    mut life_icons: Query<(&LifeIcon, &mut TextureAtlasSprite)>,
) {
    for &BurgerFinishedEvent(correct, difficulty) in ev_burger_sent.iter() {
        if correct {
            let duration = time.time_since_startup() - order.creation_time;
            score.compute_on_success(duration.as_secs_f64(), difficulty);
        } else {
            score.compute_on_failure();
            // Update life icons
            for (LifeIcon(i), mut sprite) in life_icons.iter_mut() {
                sprite.index = if *i >= score.lives { 1 } else { 0 };
            }
        }
        break;
    }
    ev_burger_sent.clear();
}
