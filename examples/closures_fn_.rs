struct MyStruct {
    text: &'static str,
    number: u32,
}

impl MyStruct {
    fn new(text: &'static str, number: u32) -> MyStruct {
        MyStruct {
            text: text,
            number: number,
        }
    }
    // We have to specify that 'self' is an argument.
    fn get_number(&self) -> u32 {
        self.number
    }
    // We can specify different kinds of ownership and mutability of self.
    fn inc_number(&mut self) {
        self.number += 1;
    }
    // There are three different types of 'self'
    fn destructor(self) {
        println!("Destructing {}", self.text);
    }
}

fn fn_function() {
    let mut obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
    let closure1 = |x: &MyStruct| x.get_number() + 3;
    assert_eq!(closure1(&obj1), 18);
    assert_eq!(closure1(&obj2), 13);
    // if
    obj1.inc_number(); // ok
    closure1(&obj1); // ok
}

fn fun_function() {
    let mut obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
    // obj1 is borrowed by the closure immutably.
    let closure2 = |x: &MyStruct| x.get_number() + obj1.get_number();
    assert_eq!(closure2(&obj2), 25);
    // We can borrow obj1 again immutably...
    assert_eq!(obj1.get_number(), 15);
    // But we can't borrow it mutably.
    obj1.inc_number();
    // closure2(&obj1);
}

fn fun_mut_function() {
    let mut obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);
    // obj1 is borrowed by the closure mutably.
    let mut closure3 = |x: &MyStruct| {
        obj1.inc_number();
        x.get_number() + obj1.get_number()
    };
    assert_eq!(closure3(&obj2), 26);
    assert_eq!(closure3(&obj2), 27);
    assert_eq!(closure3(&obj2), 28);
    // We can't borrow obj1 mutably or immutably
    //     assert_eq!(obj1.get_number(), 18);   // ERROR
    //     obj1.inc_number();
}

fn main() {
    fn_function();
    fun_function();
    fun_mut_function();
}
