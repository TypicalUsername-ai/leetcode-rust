use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T: Clone> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Clone> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_bheap_array(data: &[Option<T>], index: usize) -> Option<Self> {
        dbg!(&index);
        match data.get(index) {
            Some(opt) => match opt {
                Some(v) => Some(Self {
                    val: v.clone(),
                    left: Self::from_bheap_array(data, index * 2 + 1)
                        .map(|v| Rc::new(RefCell::new(v))),
                    right: Self::from_bheap_array(data, index * 2 + 2)
                        .map(|v| Rc::new(RefCell::new(v))),
                }),
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bheap_basic() {
        let data = vec![Some(1), Some(2), Some(3)];
        let node = TreeNode::from_bheap_array(&data, 0).unwrap();
        assert_eq!(node.val, 1);
        assert_eq!(node.left.as_ref().unwrap().borrow().val, 2);
        assert_eq!(node.right.as_ref().unwrap().borrow().val, 3);
    }

    #[test]
    fn bheap_nulls() {
        let data = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
        let node = TreeNode::from_bheap_array(&data, 0).unwrap();
        assert_eq!(node.val, 1);
        assert_eq!(node.left.as_ref().unwrap().borrow().val, 2);
        assert_eq!(node.right.as_ref().unwrap().borrow().val, 3);
        assert_eq!(
            node.right
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );
        assert_eq!(
            node.right
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            5
        );
    }
}
