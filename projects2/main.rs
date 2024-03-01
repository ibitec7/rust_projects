use std::{collections::btree_map::Range, rc::Rc};

enum List{
    Cons(i32, Rc<List>),
    Nil
}

fn main(){
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(8, Rc::new(List::Nil)))));
    let b = List::Cons(7,Rc::clone(&a));
    println!("{}",Rc::strong_count(&a));
    {
        let c = List::Cons(10, Rc::clone(&a));
        println!("{}",Rc::strong_count(&a));
    }
    println!("{}",Rc::strong_count(&a));

}