// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(v: Vec<i32>) -> Self {
        let mut iter = v.into_iter();
        let mut head = Box::new(ListNode::new(iter.next().unwrap()));
        let mut head_ref = &mut head;

        for val in iter {
            std::mem::replace(&mut head_ref.next, Some(Box::new(ListNode::new(val))));
            let next = head_ref.next.as_mut().unwrap();
            head_ref = next;
        }

        *head
    }

    pub fn to_vec(self) -> Vec<i32> {
        let mut r = vec![];
        let mut ptr = self;
        loop {
            r.push(ptr.val);
            match ptr.next {
                Some(v) => {
                    ptr = *v;
                }
                None => {
                    break;
                }
            }
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_vec_test() {
        let mut testhead = Box::new(ListNode::from_vec(vec![1, 2, 3]));

        assert_eq!(testhead.val, 1);

        testhead = testhead.next.unwrap();

        assert_eq!(testhead.val, 2);

        testhead = testhead.next.unwrap();

        assert_eq!(testhead.val, 3);
        assert_eq!(testhead.next, None);
    }
}
