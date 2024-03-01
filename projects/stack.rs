pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub trait Stack<T> {
    fn new() -> Self; // Create a new empty stack
    fn push(&mut self, item: T); // Add an element to the top of the stack
    fn pop(&mut self) -> Option<T>; // Remove and return the top element
    fn is_empty(&self) -> bool; // Check if the stack is empty
}

pub struct LLStack<T> {
    top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> for LLStack<T> {
    fn new() -> Self {
        LLStack { top: None }
    }

    fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            data: item, // Missing colon and variable name
            next: self.top.take(),
        });
        self.top = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| node.data)
    }

    fn is_empty(&self) -> bool {
        self.top.is_none()
    }
}

fn main() {
    let mut a = LLStack::<String>::new();
    a.push("Hello".to_string());
    a.push("World".to_string());
    a.push("My name is".to_string());
    a.push("Ibrahim".to_string());

    for i in range(0..5) {
        println!("{}", a.pop().unwrap_or_default());
    }
}
