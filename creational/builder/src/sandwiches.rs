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
pub enum Vegetables {
    Tomato,
    Onion,
    Corn,
}
#[derive(Clone)]
pub enum Condiments {
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
    vegetables: Option<Vec<Vegetables>>,
    condiments: Option<Vec<Condiments>>,
}

impl Sandwich {
    pub fn init() -> Self {
        Sandwich {
            bread: None,
            sauce: None,
            cheese: None,
            meat: None,
            vegetables: None,
            condiments: None,
        }
    }
    pub fn add_bread(&mut self, breads: Vec<Bread>) -> &mut Self {
        if let None = self.bread {
            self.bread = Some(breads);
        } else {
            panic!("Already bread is added");
        }
        self
    }

    pub fn add_sauce(&mut self, sauces: Vec<Sauce>) -> &mut Self {
        if let None = self.sauce {
            self.sauce = Some(sauces);
        } else {
            panic!("Already sauce is added");
        }
        self
    }

    pub fn add_cheese(&mut self, cheese: Vec<Cheese>) -> &mut Self {
        if let None = self.cheese {
            self.cheese = Some(cheese);
        } else {
            panic!("Already cheese is added");
        }
        self
    }

    pub fn add_meat(&mut self, meat: Vec<Meat>) -> &mut Self {
        if let None = self.meat {
            self.meat = Some(meat);
        } else {
            panic!("Already meat is added");
        }
        self
    }

    pub fn add_vegetables(&mut self, vegetables: Vec<Vegetables>) -> &mut Self {
        if let None = self.vegetables {
            self.vegetables = Some(vegetables);
        } else {
            panic!("Already vegetables is added");
        }
        self
    }

    pub fn add_condiments(&mut self, condiments: Vec<Condiments>) -> &mut Self {
        if let None = self.condiments {
            self.condiments = Some(condiments);
        } else {
            panic!("Already condiments is added");
        }
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
