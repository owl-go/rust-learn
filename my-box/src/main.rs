use std::ops::Deref;
use List::{Cons, Nil};
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T); //元组结构体
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
use std::rc::Rc;
fn main() {
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5, *y);

    hello("Rust");
    let m = MyBox::new("haozy");
    hello(&m);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count {}", Rc::strong_count(&a));
    }
    println!("count {}", Rc::strong_count(&a));
    // let c = Cons(3, Box::new(a));//a 已经被移动了
}
fn hello(name: &str) {
    println!("hello,{}", name);
}
