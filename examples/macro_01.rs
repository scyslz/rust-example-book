// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}
// TODO: command
// {cargo install cargo-expand}
// {cargo expand --example macro_o1}
fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
