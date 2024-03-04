#[derive (Clone)]
pub struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>,
}


impl<T> Node<T>{
    pub fn new_node(item: T) -> Node<T>{
        Node { data: item, next: None }
    }

    pub fn add(self, item: T) -> Node<T>{
        Node{ data: item, next: Some(Box::new(self)) }
    }

    pub fn extract(&self) -> T{
        self.data
    }

    pub fn iterr(&mut self) -> Option<Node<T>>{
        match self.next {
            Some(ptr) => Some(*ptr),
            None => None,
        }
    }
}