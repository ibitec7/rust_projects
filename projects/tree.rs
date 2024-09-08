use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Tree {
    pub root: Option<Box<Node>>,
}

#[derive(Clone)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, val: i32) {
        if val < self.value {
            match self.left.as_mut() {
                Some(node) => node.deref_mut().add(val),
                None => self.left =  Some(Box::new(Node { value: val, left: None, right: None })),
            }
        }
        else if val > self.value {
            match self.right.as_mut() {
                Some(node) => node.deref_mut().add(val),
                None => self.right = Some(Box::new(Node { value: val, left: None, right: None })),
            }
        }
        else {
            return;
        }
    }

    fn is_leaf(&self) -> bool {
        if self.left.is_none() && self.right.is_none() {
            return true;
        }
        else {
            return false;
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, num: i32) {
        match self.root.as_mut() {
            Some(val) => {
                val.deref_mut().add(num);
            },
            None => self.root = Some(Box::new( Node{ value: num, left: None, right: None } ))
        }
    }
}

fn main(){
    let mut tree = Tree::new();
    tree.insert(8);
    tree.insert(4);
    tree.insert(11);
    tree.insert(5);
    tree.insert(17);
    tree.insert(2);
    tree.insert(9);
}

