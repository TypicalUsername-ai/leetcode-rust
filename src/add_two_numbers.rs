use crate::structs::singly_linked_list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut rlist = Some(Box::new(ListNode::new(-1)));
    let mut end_ptr = &mut rlist;

    let mut ptr1 = &l1;
    let mut ptr2 = &l2;

    let mut carry = 0;

    while ptr1.is_some() || ptr2.is_some() || carry != 0 {
        match (&ptr1, &ptr2) {
            (None, None) => {
                std::mem::swap(end_ptr, &mut Some(Box::new(ListNode::new(carry))));
                carry = 0;
            }
            (None, Some(b)) => {
                let add_val = b.val + carry;
                std::mem::swap(end_ptr, &mut Some(Box::new(ListNode::new(add_val % 10))));
                carry = add_val / 10;
                ptr2 = &b.next;
            }
            (Some(a), None) => {
                let add_val = a.val + carry;
                std::mem::swap(end_ptr, &mut Some(Box::new(ListNode::new(add_val % 10))));
                carry = add_val / 10;
                ptr1 = &a.next;
            }
            (Some(a), Some(b)) => {
                let add_val = a.val + b.val + carry;
                std::mem::swap(end_ptr, &mut Some(Box::new(ListNode::new(add_val % 10))));
                carry = add_val / 10;
                ptr1 = &a.next;
                ptr2 = &b.next;
            }
        }
        // jump end ptr
        end_ptr = &mut end_ptr.as_mut().expect("Must exist").next;
    }

    rlist
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let res = add_two_numbers(l1, l2);

        let mut r = res.unwrap();

        let results = vec![7, 0, 8];
        for (i, digit) in results.iter().enumerate() {
            assert_eq!(&r.val, digit);
            if i != results.len() - 1 {
                r = r.next.unwrap();
            } else {
                assert!(r.next.is_none())
            }
        }
    }

    #[test]
    fn case2() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));

        let res = add_two_numbers(l1, l2);

        assert!(res.is_some());
        let data = res.unwrap();
        assert_eq!(data.val, 0);
        assert_eq!(data.next, None);
    }

    #[test]
    fn case3() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode::new(9))),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode::new(9))),
                })),
            })),
        }));

        let res = add_two_numbers(l1, l2);

        let mut r = res.unwrap();

        let results = vec![8, 9, 9, 9, 0, 0, 0, 1];
        for (i, digit) in results.iter().enumerate() {
            assert_eq!(&r.val, digit);
            if i != results.len() - 1 {
                r = r.next.unwrap();
            } else {
                assert!(r.next.is_none())
            }
        }
    }
}

// Better solution:
// pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         match (l1, l2) {
//             (None, None) => None,
//             (Some(n), None) | (None, Some(n)) => Some(n),
//             (Some(n1), Some(n2)) => {
//                 let sum = n1.val + n2.val;
//                 if sum < 10 {
//                     Some(Box::new(ListNode {
//                         val: sum,
//                         next: Solution::add_two_numbers(n1.next, n2.next)
//                     }))
//                 } else {
//                     let carry = Some(Box::new(ListNode::new(1)));
//                     Some(Box::new(ListNode {
//                         val: sum - 10,
//                         next: Solution::add_two_numbers(Solution::add_two_numbers(carry, n1.next), n2.next)
//                     }))
//                 }
//             }
//         }
//     }
