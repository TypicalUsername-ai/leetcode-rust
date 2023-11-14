use crate::structs::singly_linked_list::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut prev =
        (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr.unwrap().next.as_mut());
    prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let s = Box::new(ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let res = remove_nth_from_end(Some(s), 2);

        let t = vec![1, 2, 3, 5];

        assert_eq!(res.unwrap().to_vec(), t);
    }

    #[test]
    fn case2() {
        let s = Box::new(ListNode::new(1));

        let res = remove_nth_from_end(Some(s), 1);

        assert_eq!(res, None);
    }

    #[test]
    fn case3() {
        let s = Box::new(ListNode::from_vec(vec![1, 2]));

        let res = remove_nth_from_end(Some(s), 1);

        let t = vec![2];

        assert_eq!(res.unwrap().val, 2);
    }

    #[test]
    fn case4() {
        let s = Box::new(ListNode::from_vec(vec![1, 2]));

        let res = remove_nth_from_end(Some(s), 2);

        let t = vec![1];

        assert_eq!(res.unwrap().to_vec(), t);
    }

    #[test]
    fn case5() {
        let s = Box::new(ListNode::from_vec(vec![1]));

        let res = remove_nth_from_end(Some(s), 1);

        assert_eq!(res, None);
    }
}
