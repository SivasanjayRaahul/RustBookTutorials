mod garden;
mod guessing_game;
mod generics;
use garden::vegetable::backyard::Vegetable;

// mod reference;
// mod enums;
fn main() {
    println!("Hello, world!");
    // guessing_game::main();
    // reference::main();
    // enums::main();

    garden::front_yard::front_yard_two::lk();

    generics::main();
}

fn create_vegetable() {
    garden::vegetable::backyard::Vegetable{
        name: String::from("Carrot"),
        family_type: String::from("Root"),
    };
    let beetroot = Vegetable {
        name: String::from("Beetroot"),
        family_type: String::from("Root"),
    };
}

