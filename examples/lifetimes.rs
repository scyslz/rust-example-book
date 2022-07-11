// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_max<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    if first > second {
        return first;
    }
    return second;
}

fn get_uuid() -> &'static str {
    let s: &'static str = "hello world";
    return s;
}

fn main() {
    let first = 2; // Longer lifetime
    let first2 = 6;
    let x;
    {
        let second = 3; // Shorter lifetime
        let second2 = 4; // Shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_max(&first, &second));
        println!("{} is the first", choose_max(&second, &first));
        println!("{} is the first", choose_max(&second2, &first2));
        x = choose_max(&second2, &first2);
    };
    // println!("x is {} and addr {:p}", x, x);
    // Error, outlivee

    println!("get uuid {}",get_uuid());
}
