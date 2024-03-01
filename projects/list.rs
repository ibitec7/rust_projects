
struct Node<T>{
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T>{
    fn new(val: T) -> Box<Node<T>>{
        Box::new(Node { value: val, next: None })
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>{
    fn new() -> Self{
        LinkedList {head: None}
    }

    fn push_back(&mut self, item: T){
        let mut new_node = Node::new(item);
        let mut tail = &mut self.head();
        while let Some(ref mut node) = tail {
            tail = &mut node.next;
        }

        *tail = Some(new_node);
    }
}

fn main() {
    println!("Hello, world!");
}
