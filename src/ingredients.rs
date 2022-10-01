#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Ingredient {
    Bread,
    Steak,
    Salad,
    Tomato,
    Egg,
}

impl Ingredient {
    pub fn from_key(key: &char) -> Option<Self> {
        match key {
            'b' => Some(Self::Bread),
            's' => Some(Self::Steak),
            'd' => Some(Self::Salad),
            't' => Some(Self::Tomato),
            'e' => Some(Self::Egg),
            _ => None,
        }
    }

    pub fn atlas_key(&self, first: bool) -> usize {
        match (self, first) {
            (Ingredient::Bread, true) => 0,
            (Ingredient::Bread, false) => 1,
            (Ingredient::Steak, _) => 2,
            (Ingredient::Tomato, _) => 3,
            (Ingredient::Salad, _) => 4,
            (Ingredient::Egg, _) => 5,
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
            Menu::Uno => vec![Ingredient::Steak, Ingredient::Salad, Ingredient::Tomato, Ingredient::Egg]
        }
    }
}
