type TreeNode = crate::structs::binary_tree_node::TreeNode<i32>;

use std::cell::RefCell;
use std::rc::Rc;

fn get_leafs(root: Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) {
    if let Some(n) = root {
        let node = n.borrow();
        if node.left.is_none() && node.right.is_none() {
            leafs.push(node.val);
        } else {
            get_leafs(node.left.clone(), leafs);
            get_leafs(node.right.clone(), leafs);
        }
    }
}

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut v1 = vec![];
    let mut v2 = vec![];

    get_leafs(root1, &mut v1);
    get_leafs(root2, &mut v2);

    v1 == v2
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;
    type TreeNode = crate::structs::binary_tree_node::TreeNode<i32>;

    #[test]
    fn case1() {
        let root1 = Some(Rc::new(RefCell::new(
            TreeNode::from_bheap_array(
                &[
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(9),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4),
                ],
                0,
            )
            .unwrap(),
        )));
        let root2 = Some(Rc::new(RefCell::new(
            TreeNode::from_bheap_array(
                &[
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(7),
                    Some(4),
                    Some(2),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(9),
                    Some(8),
                ],
                0,
            )
            .unwrap(),
        )));

        assert!(leaf_similar(root1, root2))
    }

    #[test]
    fn case2() {
        let root1 = Some(Rc::new(RefCell::new(
            TreeNode::from_bheap_array(&[Some(1), Some(2), Some(3)], 0).unwrap(),
        )));
        let root2 = Some(Rc::new(RefCell::new(
            TreeNode::from_bheap_array(&[Some(1), Some(3), Some(2)], 0).unwrap(),
        )));
        assert!(!leaf_similar(root1, root2))
    }
}
