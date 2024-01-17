use std::collections::HashMap;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for i in arr {
        map.entry(i).and_modify(|i| *i += 1).or_insert(1);
    }
    let mut nums = map.values().collect::<Vec<&i32>>();
    nums.sort_unstable();
    nums.dedup();
    nums.len() == map.values().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true)
    }

    #[test]
    fn case2() {
        assert_eq!(unique_occurrences(vec![1, 2]), false)
    }

    #[test]
    fn case3() {
        assert_eq!(
            unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        )
    }
}
