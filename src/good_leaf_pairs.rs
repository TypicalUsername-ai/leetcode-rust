use crate::structs::binary_tree_node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type TreeNode = binary_tree_node::TreeNode<i32>;

fn traverse(node: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<char>, set: &mut Vec<Vec<char>>) {
    if let Some(node) = node {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            set.push(path.clone());
        } else {
            if let Some(left) = &node.borrow().left {
                path.push('L');
                traverse(Some(left.clone()), path, set);
                path.pop();
            }
            if let Some(right) = &node.borrow().right {
                path.push('R');
                traverse(Some(right.clone()), path, set);
                path.pop();
            }
        }
    }
}

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    let mut paths = vec![];
    traverse(root, &mut vec![], &mut paths);
    let mut count = 0;
    while paths.len() > 0 {
        let p1 = paths.pop().unwrap();
        for p2 in &paths {
            let mut i = 0;
            while p1.get(i) == p2.get(i) {
                i += 1;
                if i >= p1.len() || i >= p2.len() {
                    break;
                }
            }

            if (p1.len() + p2.len() - 2 * i) <= distance as usize {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_case() {
        let tree = TreeNode::from_bheap_array(&[Some(1)]).unwrap();
        let tree = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(count_pairs(tree, 1), 0)
    }

    #[test]
    fn no_repeat() {
        let tree = TreeNode::from_bheap_array(&[Some(1)]).unwrap();
        let tree = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(count_pairs(tree, 0), 0)
    }

    #[test]
    fn none_case() {
        assert_eq!(count_pairs(None, 1), 0)
    }

    #[test]
    fn basic_case() {
        let tree = TreeNode::from_bheap_array(&[Some(1), Some(2), Some(3), None, Some(4)]).unwrap();
        let tree = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(count_pairs(tree, 3), 1)
    }

    #[test]
    fn advanced_case() {
        let nodes = [
            Some(43),
            Some(32),
            Some(22),
            Some(72),
            Some(34),
            None,
            Some(28),
            None,
            None,
            None,
            Some(70),
        ];
        let tree = TreeNode::from_bheap_array(&nodes).unwrap();
        let tree = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(count_pairs(tree, 2), 0)
    }

    #[test]
    fn repeat_case() {
        let nodes = [Some(1), Some(2), Some(2), Some(1), None, Some(1)];
        let tree = TreeNode::from_bheap_array(&nodes).unwrap();
        let tree = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(count_pairs(tree, 3), 0)
    }
}
