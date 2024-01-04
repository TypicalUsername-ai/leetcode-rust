use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut hmap = HashMap::<i32, i32>::new();

    for n in nums {
        hmap.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut instrs = 0;
    // get most 3 divs so the rest is dived by 2
    for e in hmap {
        let mut twos = e.1 / 2;
        let rem = e.1.rem_euclid(2);
        let mut threes = 0;
        if rem == 1 {
            twos -= 1;
            threes += 1;
        }
        for i in 0..twos / 3 {
            twos -= 3;
            threes += 2;
        }
        if twos >= 0 && threes >= 0 {
            instrs += twos + threes;
        } else {
            return -1;
        }
    }
    instrs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
        assert_eq!(min_operations(nums), 4);
    }

    #[test]
    fn case2() {
        let nums = vec![2, 1, 2, 2, 3, 3];
        assert_eq!(min_operations(nums), -1);
    }
}
