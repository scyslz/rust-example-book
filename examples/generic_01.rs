use std::fmt::Display;

fn foo<T :Display>(name: T) {
    println!("name is {}", name)
}
fn foo01<T :Display>(name: &T) {
    println!("name is {}", name)
}

fn main() {
    let string = String::from("lz");
    // foo01(string);
    // TODO: try error
    foo(string);
    // there is a reference, so any ordered.
    let string1 = &String::from("lz");
    // auto deference
    foo(string1);
    foo01(string1);
}