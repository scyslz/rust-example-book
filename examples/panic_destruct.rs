struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        // panic=  "unwind" clean it, "abort" clean memory depends on OS

        println!("Dropping {:?}!", self.0);
    }
}
fn main() {
    let foo = Foo(1);
    panic!("Aaaaaaahhhhh!");
}