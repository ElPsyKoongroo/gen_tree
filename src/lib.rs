pub mod gen_tree;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use gen_tree::*;
    use std::rc::*;

    #[test]
    fn read() {
        let gen_tree: GenTree<i32> = GenTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));

        let root = gen_tree.get();
        let node = root.get().unwrap();
        assert_eq!(node.value, 10);
    }

    #[test]
    fn write() {
        let gen_tree: GenTree<i32> = GenTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let mut root = gen_tree.get().get().unwrap();
        let node = Rc::make_mut(&mut root);
        node.value = 14;
        assert_eq!(node.value, 14);
    }

    #[test]
    fn add_to_the_right() {
        let mut gen_tree: GenTree<i32> = GenTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let mut root = gen_tree.get_mut().get_mut().unwrap();

        let new_leaf = Leaf::new(14);

        root.set_right(new_leaf);
        let izq = (*gen_tree.get().get().unwrap()).right.get().unwrap().value;
        assert_eq!(izq, 14);
    }

    #[test]
    fn remove_to_the_right() {
        let gen_tree: GenTree<i32> = GenTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let mut root = gen_tree.get().get().unwrap();
        let node = Rc::make_mut(&mut root);
        let new_leaf = Leaf::new(14);
        node.set_right(new_leaf);

        node.set_right(Leaf::Nil);

        match root.right.get() {
            Some(_) => panic!("Unexpected Leaf"),
            None => (),
        }
    }

    #[test]
    fn turbo_right_addition() {
        let gen_tree: GenTree<i32> = GenTree::new(Node::new(10, Leaf::Nil, Leaf::Nil));
        let mut rc_actual_node = gen_tree.get().get().unwrap();
        for i in 11..=20 {
            let node = Rc::make_mut(&mut rc_actual_node);
            let new_leaf = Leaf::new(i);
            (*node).set_right(new_leaf);
            rc_actual_node = node.right.get().unwrap();
        }

        let rc_actual_node = gen_tree.get().get().unwrap();
        println!("Right: {:?}, Left: {:?}", rc_actual_node.right, rc_actual_node.left);

        let mut rc_actual_node = gen_tree.get().get().unwrap();
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
}
