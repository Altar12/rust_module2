// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Chocolate,
    Strawberry,
    Mango,
    Orange,
}
struct Drink {
    flavor: Flavor,
    fluid_ounce: f32,
}

fn print_drink(drink: &Drink) {
    match &drink.flavor {
        Flavor::Chocolate => println!("Its chococalate flavor"),
        Flavor::Strawberry => println!("Its strawberry flavor"),
        Flavor::Mango => println!("Its mango flavor"),
        Flavor::Orange => println!("Its orange flavor"),
    }
    println!("Volume is {} fluid ounce", drink.fluid_ounce);
}
fn main() {
    let drinks = vec![
        Drink {
            flavor: Flavor::Chocolate,
            fluid_ounce: 22.45,
        },
        Drink {
            flavor: Flavor::Mango,
            fluid_ounce: 30.23,
        },
        Drink {
            flavor: Flavor::Orange,
            fluid_ounce: 23.43,
        },
        Drink {
            flavor: Flavor::Strawberry,
            fluid_ounce: 33.23,
        },
    ];
    for drink in drinks.iter() {
        print_drink(drink);
    }
}
