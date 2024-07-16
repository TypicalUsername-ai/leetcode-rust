use crate::structs::binary_tree_node;
use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = binary_tree_node::TreeNode<i32>;

fn find_in_tree(
    ptr: Option<Rc<RefCell<TreeNode>>>,
    path: &mut Vec<char>,
    target: i32,
) -> Option<String> {
    if let Some(ptr) = ptr {
        if ptr.borrow().val == target {
            Some(path.iter().collect())
        } else {
            path.push('L');
            let left = find_in_tree(ptr.borrow().left.clone(), path, target);
            if left.is_some() {
                return left;
            }
            path.pop();
            path.push('R');
            let right = find_in_tree(ptr.borrow().right.clone(), path, target);
            if right.is_some() {
                right
            } else {
                path.pop();
                None
            }
        }
    } else {
        None
    }
}

pub fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    if let Some(tree) = root {
        let mut to_start = find_in_tree(Some(tree.clone()), &mut vec![], start_value).unwrap();
        let mut to_end = find_in_tree(Some(tree), &mut vec![], dest_value).unwrap();

        let len = to_start
            .chars()
            .zip(to_end.chars())
            .take_while(|(a, b)| a == b)
            .count();

        "U".repeat(to_start.len() - len) + to_end.split_at(len).1
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_case() {
        let tree =
            TreeNode::from_bheap_array(&[Some(2), Some(1), None]).expect("Error creating tree");
        let tree = Rc::new(RefCell::new(tree));
        let ans = get_directions(Some(tree), 2, 1);
        assert_eq!(ans, "L")
    }

    #[test]
    fn advanced_case() {
        let tree = TreeNode::from_bheap_array(&[
            Some(5),
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(6),
            Some(4),
        ])
        .expect("Error creating tree");
        let tree = Rc::new(RefCell::new(tree));
        let ans = get_directions(Some(tree), 3, 6);
        assert_eq!(ans, "UURL")
    }

    #[test]
    fn redundant_root_case() {
        let tree = TreeNode::from_bheap_array(&[
            Some(5),
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(6),
            Some(4),
        ])
        .expect("Error creating tree");
        let tree = Rc::new(RefCell::new(tree));
        let ans = get_directions(Some(tree), 6, 4);
        assert_eq!(ans, "UR")
    }

    #[test]
    fn null_case() {
        let ans = get_directions(None, 2, 1);
        assert_eq!(ans, "")
    }

    #[test]
    fn stress_case() {
        let ints = (0..=100000).map(|i| Some(i)).collect::<Vec<Option<i32>>>();
        let tree = TreeNode::from_bheap_array(&ints).expect("Error creating tree");
        let tree = Rc::new(RefCell::new(tree));
        let ans = get_directions(Some(tree), 88340, 100000);
        assert_eq!(ans, "UUUUUUUUUUUUUUUURLLLLRRLRLRLLLLR")
    }
}
