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

    pub fn atlas_key(&self) -> usize {
        match self {
            Ingredient::Bread => 0,
            Ingredient::Steak => 1,
            Ingredient::Tomato => 2,
            Ingredient::Salad => 3,
            Ingredient::Egg => 4,
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
