use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Copy)]
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

#[derive(Clone, Copy)]
pub enum Menu {
    Uno
}

impl Menu {
    pub fn name(&self) -> &'static str {
        match self {
            Menu::Uno => "Menu Uno"
        }
    }

    pub fn ingredients(&self) -> Vec<Ingredient> {
        match self {
            Menu::Uno => vec![Ingredient::Bread, Ingredient::Steak, Ingredient::Salad]
        }
    }
}
