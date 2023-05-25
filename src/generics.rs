struct Person<T> {
    x: T,
    y: T,
}

impl<T> Person<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Person<u8> {
    fn add(&self) -> u8 {
        self.x + self.y
    }
}

#[derive(Debug)]
struct Board<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> Board<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Board<X2, Y2>) -> Board<X1, Y2> {
        Board {
            x: self.x,
            y: other.y,
        }
    }
}

enum Level<A, B> {
    One(A),
    Two(B),
}


pub fn main() {
    let person_one = Person {
        x: 1,
        y: 2,
    };
    println!("{}", person_one.x());
    println!("{}", person_one.add());

    let board_one = Board {
        x: 1.0,
        y: 2,
    };
    let board_two = Board{
        x:3,
        y:4.0
    };
    let board_three=board_one.mixup(board_two);

    println!("{:?}",board_three);
}