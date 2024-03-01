
use std::ops::Deref;

enum List{
    Node(i32, Box<List>),
    Null
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    
    type Target = T;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

fn hello(x: &str){
    println!("Hello, {}!",x);
}

fn main() {
    let _a = List::Node(32, Box::new(List::Node(46, Box::new(List::Null))));
    let x = 5;
    let y = Box::new(x);
    let z = Box::new(String::from("Ibrahim"));
    hello(&z);      // &MyBox -> &String -> &str    ||  &Box -> &String -> &str
    drop(y);
    assert_eq!(5,x);
}
