use bevy::prelude::*;

#[derive(Clone)]
pub enum Ingredient {
    Bread,
    Steak,
    Salad
}

#[derive(Clone)]
pub struct Menu {
    pub name: &'static str,
    pub ingredients: &'static [&'static Ingredient],
    // background: String, // --> Paths to png
}

#[derive(Component)]
pub struct Order {
    pub ingredients: Vec<Ingredient>,
}

pub static MENU_UNO: Menu = Menu {
    name: "Burger",
    ingredients: &[&Ingredient::Bread, &Ingredient::Steak, &Ingredient::Salad],
};
