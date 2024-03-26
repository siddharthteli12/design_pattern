mod sandwiches;

use sandwiches::*;

fn main() {
    let sandwich = Sandwich::init()
        .add_bread(vec![Bread::Garlic])
        .add_cheese(vec![Cheese::American])
        .add_meat(vec![Meat::Chicken]).build();
}
