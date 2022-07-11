use std::borrow::Borrow;

struct A(&'static str);

struct B(&'static str);

trait Listener {
    fn listen(&self);
}

impl Listener for A {
    fn listen(&self) {
        println!("A name is {}", self.0);
    }
}

impl Listener for B {
    fn listen(&self) {
        println!("B name is {}", self.0);
    }
}

// return Box
fn get_listener(a: i32) -> Box<dyn Listener> {
    if (a > 0) {
        let a = Box::new(A("a"));
        return a;
    }
    Box::new(B("b"))
}
// only support one of type
fn get_listener01(a: i32) -> impl Listener {
    return A("a");
}
// must one specific
fn invoke_listener(lister:  impl Listener) {
    lister.listen();
}

fn main() {
    // TODO dyn
    let l0: &dyn Listener = &B("bb");
    l0.listen();

    let l1 = get_listener(1);
    l1.listen();
    let l2 = get_listener(-1);
    l2.listen();

    let l3 = get_listener01(1);
    l3.listen();
    // invoke_listener(l1.as_ref());
    // TODO Error
    // invoke_listener(l2);
    invoke_listener(l3);
    // invoke_listener(l4);
}