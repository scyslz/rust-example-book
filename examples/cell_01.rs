use std::cell::{Cell,RefCell};

struct ThingWithCell {
    counter: Cell<u8>,
}

impl ThingWithCell {
    fn new() -> ThingWithCell {
        ThingWithCell { counter: Cell::new(0) }
    }

    fn increment(&self) {
        self.counter.set(self.counter.get() + 1);
    }

    fn count(&self) -> u8 { self.counter.get() }
}

struct ThingWithRefCell {
    counter: RefCell<u8>,
}

impl ThingWithRefCell {
    fn new() -> ThingWithRefCell {
        ThingWithRefCell { counter: RefCell::new(0) }
    }

    fn increment(&self) {
        let mut counter = self.counter.borrow_mut();
        *counter = *counter + 1;
    }

    fn count(&self) -> u8 { *self.counter.borrow_mut() }
}


fn main() {
    let cell = ThingWithCell::new();
    cell.increment();
    println!("{}", cell.count());

    let cell = ThingWithRefCell::new();
    cell.increment();
    println!("{}", cell.count());
}