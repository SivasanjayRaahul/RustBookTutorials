pub fn main() {
    let mut string = String::from("SSR");

    let length = calculate_length(&string);

    change_string(&mut string);
    println!("{}", length);
    println!("{}", string);
}

fn calculate_length(string: &String) -> usize {
    return string.len();
}

fn change_string(string: &mut String) {
    string.push_str("aahul");
}