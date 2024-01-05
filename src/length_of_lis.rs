use std::collections::HashMap;

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut nmap = HashMap::<usize, i32>::new();
    for (i, n) in nums.iter().enumerate() {
        let max_prev = nmap
            .iter()
            .filter_map(|(k, v)| if nums[*k] < *n { Some(v + 1) } else { None })
            .max();
        nmap.entry(i).or_insert(max_prev.unwrap_or(1));
    }
    *nmap.values().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(input), 4);
    }

    #[test]
    fn case2() {
        let input = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(length_of_lis(input), 4);
    }

    #[test]
    fn case3() {
        let input = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(length_of_lis(input), 1);
    }
}
