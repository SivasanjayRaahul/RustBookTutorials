pub mod vegetable;

pub mod front_yard {
    //make it public to make it enable for sibling function
    use crate::garden::vegetable::backyard::Vegetable;

    // crate::garden::vegetable::backyard::Vegetable;

    pub fn plant() {
        let lemon = Vegetable {
            name: String::from("Lemon"),
            family_type: String::from("Tree"),
        };
    //super::front_yard_two::lk();
    }
    pub use super::front_yard_two;
    // front_yard_two::lk();
}


//making it public for re-exporting not for using inside sibling
pub mod front_yard_two {
    pub fn lk() {
        println!("Inside lk");
    }
}

fn add_frontyard() {
    front_yard::plant();
}