use std::{cell::RefCell, collections::BinaryHeap};

struct SeatManager {
    max_limit: i32,
    reservations: RefCell<BinaryHeap<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        let heap = BinaryHeap::from((1..=n).into_iter().collect::<Vec<i32>>());
        Self {
            reservations: RefCell::new(heap),
            max_limit: n + 1,
        }
    }

    fn reserve(&self) -> i32 {
        self.max_limit
            - self
                .reservations
                .borrow_mut()
                .pop()
                .expect("Always called if there are places")
    }

    fn unreserve(&self, seat_number: i32) {
        self.reservations
            .borrow_mut()
            .push(self.max_limit - seat_number);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mgr = SeatManager::new(5);

        assert_eq!(mgr.reserve(), 1);
        assert_eq!(mgr.reserve(), 2);
        mgr.unreserve(2);
        assert_eq!(mgr.reserve(), 2);
        assert_eq!(mgr.reserve(), 3);
        assert_eq!(mgr.reserve(), 4);
        assert_eq!(mgr.reserve(), 5);
        mgr.unreserve(5);
    }
}
