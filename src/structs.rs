#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

impl User {
    fn printName(&self) {
        println!("{}", self.name);
    }

    fn is_older(&self, user_two: &User) -> bool {
        self.age > user_two.age
    }

    fn new(name: String, age: u8, active: bool) -> Self {
        Self {
            name,
            age,
            active,
        }
    }
}

pub(crate) fn main() {
    let user = User {
        name: String::from("SSR"),
        active: true,
        age: 100,
    };//immutable

    let mut user_one = User {
        name: String::from("Siva"),
        active: false,
        age: 100,
    };//mutable

    user_one.age = 99;

    let user_two = User {
        name: user.name,
        active: user.active,
        age: user.age,
    };

    // println!("{}", user.name); error
    println!("{}", user.age);//valid

    let user_three = User {
        name: String::from("Raahul"),
        ..user_two
    };


    println!("{:?}", user_three);
}

fn signUp(name: String, age: u8) {
    User {
        name,
        age,
        active: true,
    };
}