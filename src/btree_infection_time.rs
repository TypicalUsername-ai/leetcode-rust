use std::borrow::BorrowMut;
use std::{cell::RefCell, rc::Rc};
type TreeNode = crate::structs::binary_tree_node::TreeNode<i32>;

use std::collections::{HashMap, HashSet};

pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let mut graph = HashMap::new();
    map_graph(root, None, &mut graph);
    let mut seen = HashSet::new();
    let mut edge = vec![start];
    let mut time = -1;
    while seen.len() != graph.len() {
        let mut nedge = HashSet::new();
        while let Some(e) = edge.pop() {
            if seen.insert(e) {
                nedge.extend(graph.get(&e).unwrap())
            }
        }
        edge = nedge.into_iter().collect();
        time += 1;
    }
    time
}

fn map_graph(
    root: Option<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
    graph: &mut HashMap<i32, Vec<i32>>,
) {
    if let Some(v) = root.clone() {
        let node = v.borrow();
        let mut vec = [parent, node.left.clone(), node.right.clone()]
            .into_iter()
            .filter_map(|e| e.map(|v| v.borrow().val))
            .collect();
        let entry = graph.insert(node.val, vec);
        map_graph(node.left.clone(), root.clone(), graph);
        map_graph(node.right.clone(), root.clone(), graph);
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn case1() {
        let root = Rc::new(RefCell::new(
            TreeNode::from_bheap_array(&[
                Some(1),
                Some(5),
                Some(3),
                None,
                Some(4),
                Some(10),
                Some(6),
                None,
                None,
                Some(9),
                Some(2),
            ])
            .unwrap(),
        ));
        assert_eq!(amount_of_time(Some(root), 3), 4)
    }

    #[test]
    fn case2() {
        assert_eq!(
            amount_of_time(Some(Rc::new(RefCell::new(TreeNode::new(1)))), 1),
            0
        )
    }
}
