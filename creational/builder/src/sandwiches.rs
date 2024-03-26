#![allow(dead_code)]

#[derive(Clone, Debug)]
pub enum Bread {
    Garlic,
    Italic,
    Indian,
}
#[derive(Clone, Debug)]
pub enum Sauce {
    Chilli,
    Tomato,
    Onion,
}
#[derive(Clone, Debug)]
pub enum Cheese {
    American,
    Cheddar,
}
#[derive(Clone, Debug)]
pub enum Meat {
    Chicken,
    Turkey,
    Fish,
}
#[derive(Clone, Debug)]
pub enum Vegetable {
    Tomato,
    Onion,
    Corn,
}
#[derive(Clone, Debug)]
pub enum Condiment {
    Pepper,
    Olive,
    Honey,
}

#[derive(Clone, Debug)]
pub struct SandwichBuilder {
    bread: Option<Vec<Bread>>,
    sauce: Option<Vec<Sauce>>,
    cheese: Option<Vec<Cheese>>,
    meat: Option<Vec<Meat>>,
    vegetable: Option<Vec<Vegetable>>,
    condiment: Option<Vec<Condiment>>,
}

impl SandwichBuilder {
    pub fn new() -> Self {
        SandwichBuilder {
            bread: None,
            sauce: None,
            cheese: None,
            meat: None,
            vegetable: None,
            condiment: None,
        }
    }

    pub fn add_bread(mut self, bread: Vec<Bread>) -> Self {
        self.bread.get_or_insert(bread);
        self
    }

    pub fn add_sauce(mut self, sauce: Vec<Sauce>) -> Self {
        self.sauce.get_or_insert(sauce);
        self
    }

    pub fn add_cheese(mut self, cheese: Vec<Cheese>) -> Self {
        self.cheese.get_or_insert(cheese);
        self
    }

    pub fn add_meat(mut self, meat: Vec<Meat>) -> Self {
        self.meat.get_or_insert(meat);
        self
    }

    pub fn add_vegetables(mut self, vegetable: Vec<Vegetable>) -> Self {
        self.vegetable.get_or_insert(vegetable);
        self
    }

    pub fn add_condiments(mut self, condiment: Vec<Condiment>) -> Self {
        self.condiment.get_or_insert(condiment);
        self
    }

    pub fn build(self) -> Sandwich {
        Sandwich {
            bread: self.bread,
            sauce: self.sauce,
            cheese: self.cheese,
            meat: self.meat,
            vegetable: self.vegetable,
            condiment: self.condiment,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sandwich {
    bread: Option<Vec<Bread>>,
    sauce: Option<Vec<Sauce>>,
    cheese: Option<Vec<Cheese>>,
    meat: Option<Vec<Meat>>,
    vegetable: Option<Vec<Vegetable>>,
    condiment: Option<Vec<Condiment>>,
}

impl Sandwich {
    pub fn  builder() -> SandwichBuilder {
        SandwichBuilder::new()
    }
}