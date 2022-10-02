#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Ingredient {
    Bread,
    Steak,
    Tomato,
    Salad,
    Egg,
    Pickles,
    Avocado,
    Cheese,
    Bacon,
    Jalapeno,
    Chicken,
    Mushrooms,
    Onions,
    Mayo,
    Ketchup,
}

impl Ingredient {
    pub fn from_key(key: &char) -> Option<Self> {
        match key {
            'b' => Some(Self::Bread),
            's' => Some(Self::Steak),
            'l' => Some(Self::Salad),
            't' => Some(Self::Tomato),
            'e' => Some(Self::Egg),
            'p' => Some(Self::Pickles),
            'v' => Some(Self::Avocado),
            'c' => Some(Self::Cheese),
            'a' => Some(Self::Bacon),
            'j' => Some(Self::Jalapeno),
            'k' => Some(Self::Chicken),
            'h' => Some(Self::Mushrooms),
            'o' => Some(Self::Onions),
            'y' => Some(Self::Mayo),
            'u' => Some(Self::Ketchup),
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
            (Ingredient::Pickles, _) => 6,
            (Ingredient::Avocado, _) => 7,
            (Ingredient::Cheese, _) => 8,
            (Ingredient::Bacon, _) => 9,
            (Ingredient::Jalapeno, _) => 10,
            (Ingredient::Chicken, _) => 11,
            (Ingredient::Mushrooms, _) => 12,
            (Ingredient::Onions, _) => 13,
            (Ingredient::Mayo, _) => 14,
            (Ingredient::Ketchup, _) => 15,
        }
    }

    pub fn key(&self) -> char {
        match self {
            Ingredient::Bread => 'b',
            Ingredient::Steak => 's',
            Ingredient::Salad => 'l',
            Ingredient::Tomato => 't',
            Ingredient::Egg => 'e',
            Ingredient::Pickles => 'p',
            Ingredient::Avocado => 'v',
            Ingredient::Cheese => 'c',
            Ingredient::Bacon => 'a',
            Ingredient::Jalapeno => 'j',
            Ingredient::Chicken => 'k',
            Ingredient::Mushrooms => 'h',
            Ingredient::Onions => 'o',
            Ingredient::Mayo => 'y',
            Ingredient::Ketchup => 'u',
        }
    }

    pub fn name(&self) -> String {
        match self {
            Ingredient::Bread => "Bread",
            Ingredient::Steak => "Steak",
            Ingredient::Salad => "Lettuce",
            Ingredient::Tomato => "Tomato",
            Ingredient::Egg => "Egg",
            Ingredient::Pickles => "Pickles",
            Ingredient::Avocado => "Avocado",
            Ingredient::Cheese => "Cheese",
            Ingredient::Bacon => "Bacon",
            Ingredient::Jalapeno => "Jalapeños",
            Ingredient::Chicken => "Chicken",
            Ingredient::Mushrooms => "Mushrooms",
            Ingredient::Onions => "Onions",
            Ingredient::Mayo => "Mayonnaise",
            Ingredient::Ketchup => "Ketchup",
        }
        .to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Menu {
    Uno,
}

impl Menu {
    pub fn name(&self) -> &'static str {
        match self {
            Menu::Uno => "Menu Uno",
        }
    }

    pub fn ingredients(&self) -> Vec<Ingredient> {
        match self {
            Menu::Uno => vec![
                Ingredient::Steak,
                Ingredient::Salad,
                Ingredient::Tomato,
                Ingredient::Egg,
                Ingredient::Pickles,
                Ingredient::Avocado,
                Ingredient::Cheese,
                Ingredient::Bacon,
                Ingredient::Jalapeno,
                Ingredient::Chicken,
                Ingredient::Mushrooms,
                Ingredient::Onions,
                Ingredient::Mayo,
                Ingredient::Ketchup,
            ],
        }
    }
}
