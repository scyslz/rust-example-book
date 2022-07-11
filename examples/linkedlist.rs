use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Consume a list, and return the same list with a new element at its front
fn prepend(list: List, elem: u32) -> List {
    // `Cons` also has type List
    Cons(elem, Box::new(list))
}

// Return the length of the list
fn len(list: &List) -> u32 {
    // `self` has to be matched, because the behavior of this method
    // depends on the variant of `self`
    // `self` has type `&List`, and `*self` has type `List`, matching on a
    // concrete type `T` is preferred over a match on a reference `&T`
    // after Rust 2018 you can use self here and tail (with no ref) below as well,
    // rust will infer &s and ref tail.
    // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    match *list {
        // Can't take ownership of the tail, because `self` is borrowed;
        // instead take a reference to the tail
        Cons(_, ref tail) => 1 + len(tail),
        // Base Case: An empty list has zero length
        Nil => 0
    }
}


// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }
    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = prepend(list, 1);
    list = prepend(list, 2);
    list = prepend(list, 3);

    // Show the final state of the list
    println!("linked list has length: {}", len(&list));
    println!("{}", list.stringify());
}
