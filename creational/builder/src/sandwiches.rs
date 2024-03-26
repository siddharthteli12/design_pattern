#[derive(Clone)]
pub enum Bread {
    Garlic,
    Italic,
    Indian,
}
#[derive(Clone)]
pub enum Sauce {
    Chilli,
    Tomato,
    Onion,
}
#[derive(Clone)]
pub enum Cheese {
    American,
    Cheddar,
}
#[derive(Clone)]
pub enum Meat {
    Chicken,
    Turkey,
    Fish,
}
#[derive(Clone)]
pub enum Vegetable {
    Tomato,
    Onion,
    Corn,
}
#[derive(Clone)]
pub enum Condiment {
    Pepper,
    Olive,
    Honey,
}

#[derive(Clone)]
pub struct Sandwich {
    bread: Option<Vec<Bread>>,
    sauce: Option<Vec<Sauce>>,
    cheese: Option<Vec<Cheese>>,
    meat: Option<Vec<Meat>>,
    vegetable: Option<Vec<Vegetable>>,
    condiment: Option<Vec<Condiment>>,
}

impl Sandwich {
    pub fn init() -> Self {
        Sandwich {
            bread: None,
            sauce: None,
            cheese: None,
            meat: None,
            vegetable: None,
            condiment: None,
        }
    }

    pub fn add_bread(&mut self, bread: Vec<Bread>) -> &mut Self {
        self.bread.get_or_insert(bread);
        self
    }

    pub fn add_sauce(&mut self, sauce: Vec<Sauce>) -> &mut Self {
        self.sauce.get_or_insert(sauce);
        self
    }

    pub fn add_cheese(&mut self, cheese: Vec<Cheese>) -> &mut Self {
        self.cheese.get_or_insert(cheese);
        self
    }

    pub fn add_meat(&mut self, meat: Vec<Meat>) -> &mut Self {
        self.meat.get_or_insert(meat);
        self
    }

    pub fn add_vegetables(&mut self, vegetable: Vec<Vegetable>) -> &mut Self {
        self.vegetable.get_or_insert(vegetable);
        self
    }

    pub fn add_condiments(&mut self, condiment: Vec<Condiment>) -> &mut Self {
        self.condiment.get_or_insert(condiment);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
