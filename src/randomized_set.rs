use rand::{thread_rng, Rng};
use std::{cell::RefCell, collections::HashSet};

struct RandomizedSet {
    set: RefCell<HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            set: RefCell::new(HashSet::new()),
        }
    }

    fn insert(&self, val: i32) -> bool {
        self.set.borrow_mut().insert(val)
    }

    fn remove(&self, val: i32) -> bool {
        self.set.borrow_mut().remove(&val)
    }

    fn get_random(&self) -> i32 {
        *self
            .set
            .borrow()
            .iter()
            .take(thread_rng().gen_range(1, self.set.borrow().len() + 1))
            .last()
            .unwrap()
    }
}

/*
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));
    }
}
