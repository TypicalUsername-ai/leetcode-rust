use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();

    let mut ret = vec![-1, -1];

    for (i, num) in nums.iter().enumerate() {
        let old = map.insert(num.to_owned(), i as i32);
        if num * 2 == target && old.is_some() {
            ret = vec![i as i32, old.unwrap()];
            break;
        }
        let remainder = &target - num;
        if let Some(key2) = map.get(&remainder) {
            if &(i as i32) == key2 {
                continue;
            }
            ret = vec![i as i32, key2.to_owned()];
            break;
        }
    }

    ret.sort();
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
