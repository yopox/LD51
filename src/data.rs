use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub enum Ingredient {
    Bread,
    Steak,
    Salad,
}

impl Ingredient {
    pub fn from_key(key: &char) -> Option<Self> {
        match key {
            'b' => Some(Self::Bread),
            's' => Some(Self::Steak),
            'd' => Some(Self::Salad),
            _ => None,
        }
    }

    pub fn atlas_key(&self) -> usize {
        match self {
            Ingredient::Bread => 0,
            Ingredient::Steak => 1,
            Ingredient::Salad => 3,
        }
    }
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
