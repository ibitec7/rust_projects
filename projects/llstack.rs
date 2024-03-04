mod node;

use node::Node;

struct LLStack<T>{
    size: i32,
    head: Option<Node<T>>,
}

impl<T> LLStack<T>{
    fn default(&mut self) -> Self{
        LLStack { size: 0, head: None }
    }

    fn push(&mut self, item: T){
        match self.head{
            Some(ptr) => {
                let node = ptr;
                self.head = Some(node.add(item));
                self.size = self.size + 1;
            } 
            None => {
                self.head = Some(Node::new_node(item));
                self.size = 1;
            }
        }
    }

    fn pop(&mut self) -> T{
        match self.head{
            Some(ptr) => {
                let node = ptr;
                let item = node.extract();
                self.head = node.iterr();
                drop(node);
                return item;
            },
            None => panic!("Can not pop an empty Stack"),
        }
    }
}

fn main(){
    println!("Hello World");
}