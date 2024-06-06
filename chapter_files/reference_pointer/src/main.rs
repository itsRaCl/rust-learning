use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(12, Rc::new(Cons(13, Rc::new(Nil)))));

    let b = Cons(12, Rc::clone(&a));
    let c= Cons(12, Rc::clone(&a));
}
