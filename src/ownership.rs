pub fn main() {
    let var1 = 10;
    let var2 = var1; //Stack copy

    let str1 = "SSR";
    let str2 = str1;

    let string1 = String::from("SSR");
    let string2 = string1;//Shallow Copy
    //println!("{}",string1); error //string1 moved

    let string3 = string2.clone();// Copy heap data too
    println!("{}", string2);
}