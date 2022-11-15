pub mod gen_tree;
pub mod node;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use gen_tree::*;
    use node::{Node, Leaf};

    #[test]
    fn read() {
        let gen_tree: BinTree<i32> = BinTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));

        let root = gen_tree.get().unwrap();
        assert_eq!(root.value, 10);
    }

    #[test]
    fn write() {
        let mut gen_tree: BinTree<i32> = BinTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let root = gen_tree.get_mut().unwrap();
        root.value = 14;
        assert_eq!(root.value, 14);
    }

    #[test]
    fn add_to_the_right() {
        let mut gen_tree: BinTree<i32> = BinTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let root = gen_tree.get_mut().unwrap();

        let new_leaf = Leaf::new(14);

        root.set_right(new_leaf);
        let izq = (*gen_tree.get().unwrap()).right.get().unwrap().value;
        assert_eq!(izq, 14);
    }

    
    #[test]
    fn remove_to_the_right() {
        let mut gen_tree: BinTree<i32> = BinTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let root = gen_tree.get_mut().unwrap();
        let new_leaf = Leaf::new(14);
        root.set_right(new_leaf);

        
        match root.right.get() {
            Some(a) => assert_eq!(a.value, 14),
            None => panic!("Unexpected Nil"),
        }

        root.set_right(Leaf::Nil);

        match root.right {
            Leaf::Value(_) => panic!("Unexpected Nil"),
            Leaf::Nil => ()
        }
    }

    
    #[test]
    fn turbo_right_addition() {
        let mut gen_tree: BinTree<i32> = BinTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let mut rc_actual_node = gen_tree.get_mut().unwrap();
        for i in 11..=20 {
            let new_leaf = Leaf::new(i);
            rc_actual_node.set_right(new_leaf);
            rc_actual_node = rc_actual_node.right.get_mut().unwrap();
        }


        let mut rc_actual_node = gen_tree.get().unwrap();
        for i in 11..=20 {
            let value = rc_actual_node.value;
            assert_eq!(value, i - 1);
            let next_leaf = &rc_actual_node.right;
            rc_actual_node = match next_leaf.get() {
                Some(a) => a,
                None => {
                    panic!("Panic at {}, actual value: {}", i, value)
                }
            }
        }
    }


    #[test]
    fn width_order_add_and_read(){
        use std::collections::VecDeque;
        let mut tree: BinTree<u32> = BinTree::new(Node::new(1, Leaf::Nil, Leaf::Nil));
        let mut nodes = VecDeque::new();
        nodes.push_back(tree.get_mut().unwrap());

        for i in (3..=15).step_by(2) {
            let actual_node = nodes.pop_front().unwrap();
            actual_node.set_left(Leaf::new(i-1));
            actual_node.set_right(Leaf::new(i));

            nodes.push_back(actual_node.left.get_mut().unwrap());
            nodes.push_back(actual_node.right.get_mut().unwrap());
        }

        let mut nodes = VecDeque::new();
        nodes.push_back(tree.get().unwrap());
        let mut counter = 1;
        while !nodes.is_empty() {
            let actual_node = nodes.pop_front().unwrap();
            assert_eq!(counter, actual_node.value);
            counter += 1;
            match actual_node.left.get() {
                Some(node) => nodes.push_back(node),
                None => {}
            };

            match actual_node.right.get() {
                Some(node) => nodes.push_back(node),
                None => {}
            }
        }

    }


}
