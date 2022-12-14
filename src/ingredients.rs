use std::cmp::min;

use rand::prelude::*;

use crate::audio::SFX;
use crate::restaurant::MENU_SIZE;

#[derive(Clone, Eq, PartialEq, Copy, Hash)]
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
    pub fn is_meat(&self) -> bool {
        match self {
            Ingredient::Steak | Ingredient::Chicken => true,
            _ => false
        }
    }

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
            Ingredient::Jalapeno => "Jalape??os",
            Ingredient::Chicken => "Chicken",
            Ingredient::Mushrooms => "Mushrooms",
            Ingredient::Onions => "Onions",
            Ingredient::Mayo => "Mayo",
            Ingredient::Ketchup => "Ketchup",
        }
        .to_string()
    }

    pub fn sfx(&self) -> SFX {
        match self {
            Ingredient::Bread => SFX::Bread,
            Ingredient::Steak => SFX::Meat,
            Ingredient::Tomato => SFX::Vegetable,
            Ingredient::Salad => SFX::Lettuce,
            Ingredient::Egg => SFX::Vegetable,
            Ingredient::Pickles => SFX::Vegetable,
            Ingredient::Avocado => SFX::Vegetable,
            Ingredient::Cheese => SFX::Sauce,
            Ingredient::Bacon => SFX::Meat,
            Ingredient::Jalapeno => SFX::Vegetable,
            Ingredient::Chicken => SFX::Meat,
            Ingredient::Mushrooms => SFX::Vegetable,
            Ingredient::Onions => SFX::Vegetable,
            Ingredient::Mayo => SFX::Sauce,
            Ingredient::Ketchup => SFX::Sauce,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Menu {
    Uno,
}

const MAX_SIZE_OF_BURGER: usize = 10;

impl Menu {
    // pub fn name(&self) -> &'static str {
    //     match self {
    //         Menu::Uno => "Menu Uno",
    //     }
    // }

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

    pub fn generate_order(&self, ingredients: &Vec<Ingredient>) -> Vec<Ingredient> {
        return match self {
            Menu::Uno => {
                let mut rng = thread_rng();

                // Choose a meat for the burger
                let meat = if ingredients.contains(&Ingredient::Chicken) && ingredients.contains(&Ingredient::Steak) {
                    if random() { Ingredient::Chicken } else { Ingredient::Steak }
                } else {
                    *ingredients.iter().find(|i| i.is_meat()).unwrap()
                };

                // random_ingredients are the ingredients that will be chosen at random
                let mut random_ingredients: Vec<Ingredient> = ingredients
                    .into_iter()
                    // Remove the special ingredients that we don't want interfering with our stuff
                    .filter(|i| match i {
                        Ingredient::Bread => false,
                        Ingredient::Steak => false,
                        Ingredient::Chicken => false,
                        Ingredient::Mayo => false,
                        Ingredient::Ketchup => false,
                        _ => true,
                    })
                    .copied()
                    .collect();

                // A bit of sauce intelligence to determine how much sauce we are going to put
                let possible_ketchup = ingredients.contains(&Ingredient::Ketchup);
                let possible_mayo = ingredients.contains(&Ingredient::Mayo);
                let is_there_sauce = random() && (possible_ketchup || possible_mayo);
                let nb_sauces = if is_there_sauce { 1 } else { 0 };

                // Double every ingredient
                let ri: Vec<Ingredient> = random_ingredients.iter().copied().collect();
                random_ingredients.extend(ri.into_iter());

                // Possible double meat
                random_ingredients.push(meat);

                // Choose a number of ingredients
                // We guard this otherwise rand fires a runtime error
                let nb = if random_ingredients.len() <= 0 {
                    0
                } else {
                    // The maximum number of ingredients that is possible to generate in this configuration
                    // MAX_SIZE_OF_BURGER - nb_bread - nb_meat_inserted_at_the_end - nb_sauces
                    let max_nb_ingredients = MAX_SIZE_OF_BURGER - 2 - 1 - nb_sauces;
                    // We use a linear distribution, ie
                    //      P(x) = (x + 1) / k
                    // where k is a normalisation constant
                    //       1 avoids the weight 0 for 0.
                    // And we go up further one, because it is removed from the range (thus '+ 2')
                    let weights = 1..min(random_ingredients.len(), max_nb_ingredients) + 2;
                    let nb_dist = rand::distributions::WeightedIndex::new(weights).unwrap();
                    // We sample in that dist
                    rng.sample(nb_dist)
                };

                // We chose nb ingredients from the possible ingredients
                let mut recipe: Vec<Ingredient> = random_ingredients
                    .choose_multiple(&mut rng, nb)
                    .into_iter()
                    .copied()
                    .collect();

                // Push the necessary meat at a random index
                recipe.push(meat);
                recipe.shuffle(&mut rng);

                // Triple meat possibility
                if random::<f32>() < 0.1 && recipe.len() < MAX_SIZE_OF_BURGER - 2 {
                    recipe.push(meat);
                    recipe.shuffle(&mut rng);
                }

                // Add maybe some sauces on top of it
                if is_there_sauce {
                    if !possible_mayo || (possible_ketchup && possible_mayo && random()) {
                        recipe.push(Ingredient::Ketchup)
                    } else {
                        recipe.push(Ingredient::Mayo)
                    }
                }

                // Add the bread on top and at bottom
                recipe.insert(0, Ingredient::Bread);
                recipe.push(Ingredient::Bread);

                recipe
            }
        };
    }

    pub fn basic_ingredients(&self, madness_mode: bool) -> Vec<Ingredient> {
        let ingredients_nb = if madness_mode { MENU_SIZE } else { 1 };
        match self {
            Menu::Uno => {
                let mut ingredients = vec![Ingredient::Bread];
                ingredients.push(if random() { Ingredient::Steak } else { Ingredient::Chicken });
                let additional_ingredient =
                    vec![Ingredient::Salad, Ingredient::Ketchup, Ingredient::Cheese]
                        .iter()
                        .choose(&mut thread_rng())
                        .copied()
                        .unwrap();
                ingredients.push(additional_ingredient);
                while ingredients.len() <= ingredients_nb {
                    let ingredient = Menu::Uno.ingredients()
                        .iter()
                        .filter(|i| !ingredients.contains(*i))
                        .choose(&mut thread_rng())
                        .unwrap()
                        .clone();
                    ingredients.push(ingredient);
                }
                ingredients
            }
        }
    }
}
