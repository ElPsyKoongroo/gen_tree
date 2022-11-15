use std::boxed::Box;

#[derive(Clone, Debug)]
pub struct Node<T: std::fmt::Debug + Clone + std::cmp::Eq + Copy> {
    pub right: Leaf<T>,
    pub left : Leaf<T>,
    pub value: T
}

impl<T: std::fmt::Debug + Clone + std::cmp::Eq + Copy> Node<T> {
   pub fn new(value: T, right: Leaf<T>, left: Leaf<T>) -> Node<T>{
       Node {
           value,
           right,
           left
       }
   }

   pub fn get(&self) -> &T {
       &self.value
   }

   pub fn set_right(&mut self, a: Leaf<T>) {
       self.right = a;
   }

   pub fn set_left(&mut self, a: Leaf<T>) {
       self.left = a;
   }
}

#[derive(Clone, Debug)]
pub enum Leaf<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> {
   Value(Box<Node<T>>),
   Nil
}

impl<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> Leaf<T> {
    
    pub fn new(value: T) -> Leaf<T>{
        Leaf::Value(Box::new(Node::new(value, Leaf::Nil, Leaf::Nil)))
    }

    pub fn get(&self) -> Option<&Node<T>> {
        match self {
            Leaf::Value(val) => Some(&val),
            Leaf::Nil => None
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut Node<T>> {
        match self {
            Leaf::Value(val) => Some(val.as_mut()),
            Leaf::Nil => None
        }
    }

    pub fn get_value(&self) -> Option<T> {
        match self {
            Leaf::Value(val) => Some(val.value),
            Leaf::Nil => None
        }
    }
}
