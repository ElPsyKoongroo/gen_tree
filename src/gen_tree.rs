use crate::node::*;

#[derive(Clone)]
pub struct BinTree<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> {
    root: Leaf<T>
}

impl<T: std::fmt::Debug + std::cmp::Eq + Clone + Copy> BinTree<T> {
    pub fn new(root_value: Node<T>) -> BinTree<T> {
        BinTree {
            root: Leaf::Value(Box::new(root_value)),
        }
    }

    pub fn get(&self) -> Option<&Node<T>> {
        self.root.get()
    }

    pub fn get_mut(&mut self) -> Option<&mut Node<T>> {
        self.root.get_mut()
    }

}
