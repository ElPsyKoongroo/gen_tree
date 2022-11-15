use std::rc::Rc;


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
   Value(Rc<Node<T>>),
   Nil
}

impl<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> Leaf<T> {
    
    pub fn new(value: T) -> Leaf<T>{
        Leaf::Value(Rc::new(Node::new(value, Leaf::Nil, Leaf::Nil)))
    }

    pub fn get(&self) -> Option<Rc<Node<T>>> {
        match self {
            Leaf::Value(val) => Some(Rc::clone(&val)),
            Leaf::Nil => None
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut Node<T>> {
        match self {
            Leaf::Value(ref mut val) => Rc::get_mut(val),
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

#[derive(Clone)]
pub struct GenTree<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> {
    root: Leaf<T>
}

impl<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> GenTree<T> {
    pub fn new(root_value: Node<T>) -> GenTree<T> {
        GenTree {
            root: Leaf::Value(Rc::new(root_value)),
        }
    }

    pub fn get(&self) -> &Leaf<T> {
        &self.root 
    }

    pub fn get_mut(&mut self) -> &mut Leaf<T> {
        &mut self.root
    }

}
