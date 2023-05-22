fn main() {
    let x = 10;
    //x = 11; error

    let mut y = 10;
    y = 11;

    const LEVELS: i8 = 3;
    // const LEVELS = 3; type not specified
    //LEVELS = 1; Immutable

    let var = 10;
    let var = 11;
    let var = "SSR";

    //var = 10; error

    let mut a = 10;
    a = 11;//mutable
    let a = 9;
    //a=11; immutable error

    let mut a = "SSE";


    //tuple
    let tup = (100, 1.0, "SSR");
    let tup: (i8, f32, &str) = (100, 1.0, "SSR");

    //Array
    let arr = [1, 2, 3, 4];
    let arr = [3; 5];
}