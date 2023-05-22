#[derive(Debug)]
enum Color {
    RED(u8),
    BLUE(u8),
    ORGYEL(u8, u8),
}

impl Color {
    fn print(&self) {
        println!("{:?}", self);
    }
}

pub(crate) fn main() {
    let color = Color::BLUE(10);
    color.print();
}