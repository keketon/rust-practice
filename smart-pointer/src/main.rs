#![allow(unused)]
/*
cannot compile

enum List {
  Cons(i32, List),
  Nil,
}
*/

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.0
  }
}

use std::ops::Deref;
use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
