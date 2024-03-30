struct Node{
    item: i32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(num: i32) -> Self {
        Node{ item: num, next: None }
    }
}

struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList {
    fn new(num: i32) -> Self{
        let a = Some(Box::new(Node::new(num)));
        LinkedList{ head: a }
    }

    fn push_back(&mut self, num: i32) {
        let new_node = Node{ item: num, next: self.head.take()};
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<i32>{
        self.head.take().map( |node| {
            let new_node = *node;
            self.head = new_node.next;
            new_node.item
        })
    }

    fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(ref node) = current {
            println!("{}", node.item);
            current = node.next.as_ref();
        }
        println!("None");
    }
}

fn main(){
    let x: i32 = 5;
    let mut a = LinkedList::new(x);
    a.print();
    a.push_back(4);
    a.push_back(7);
    a.push_back(13);
    a.print();
    let _b = a.pop();
    a.print();

}