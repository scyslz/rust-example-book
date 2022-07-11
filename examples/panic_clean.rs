// Re-implementation of integer division (/)
struct A();
impl Drop for A {
    fn drop(&mut self) {
        println!("drop A");
    }
}

fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// The `main` task
fn main() {
    // Heap allocated integer
    let _x = Box::new(0i32);
    let a = &A();

    // This operation will trigger a task failure
    division(3, 0);

    println!("This point won't be reached!");
    println!("{:p}", a);

    // `_x` should get destroyed at this point
}
// rustc panic.rs && valgrind ./panic