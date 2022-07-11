// Globals are declared outside all other scopes.
static mut LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    unsafe { println!("This is {}", LANGUAGE); }
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5; //
    unsafe { LANGUAGE = "ddf"; }
    // FIXME ^ Comment out this line
    let _a = 12;
    let b_b = 12;
}
