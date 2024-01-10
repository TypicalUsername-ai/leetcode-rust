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

    pub fn to_heaped_vec(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
        let mut data = vec![];
        Self::to_sorted_slice_inner(root, &mut data);
        data
    }

    fn to_sorted_slice_inner(ptr: Option<Rc<RefCell<TreeNode<T>>>>, arr: &mut Vec<T>) {
        if let Some(v) = ptr {
            let node = v.borrow();

            arr.push(node.val.clone());
            Self::to_sorted_slice_inner(node.left.clone(), arr);
            Self::to_sorted_slice_inner(node.right.clone(), arr);
        }
    }

    pub fn from_bheap_array(data: &[Option<T>]) -> Option<Self> {
        Self::from_bheap_array_inner(data, 0)
    }

    fn from_bheap_array_inner(data: &[Option<T>], index: usize) -> Option<Self> {
        // dbg!(&index);
        match data.get(index) {
            Some(opt) => opt.as_ref().map(|v| Self {
                    val: v.clone(),
                    left: Self::from_bheap_array_inner(data, index * 2 + 1)
                        .map(|v| Rc::new(RefCell::new(v))),
                    right: Self::from_bheap_array_inner(data, index * 2 + 2)
                        .map(|v| Rc::new(RefCell::new(v))),
                }),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn bheap_basic() {
        let data = vec![Some(1), Some(2), Some(3)];
        let node = Rc::new(RefCell::new(TreeNode::from_bheap_array(&data).unwrap()));
        assert_eq!(TreeNode::to_heaped_vec(Some(node)), [1, 2, 3]);
    }

    #[test]
    fn bheap_nulls() {
        let data = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
        let node = Rc::new(RefCell::new(TreeNode::from_bheap_array(&data).unwrap()));
        assert_eq!(TreeNode::to_heaped_vec(Some(node)), [1, 2, 3, 4, 5]);
    }
}
