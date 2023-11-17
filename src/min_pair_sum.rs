fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    let mut iter = sorted.iter();
    let mut back_iter = sorted.iter().rev();
    let mut max = 0;
    for i in 0..(nums.len() / 2) {
        let a = iter.next().unwrap();
        let b = back_iter.next().unwrap();
        if a + b > max {
            max = a + b;
        } else {
            continue;
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![3, 5, 2, 3];

        assert_eq!(min_pair_sum(input), 7);
    }

    #[test]
    fn case2() {
        let input = vec![3, 5, 4, 2, 4, 6];

        assert_eq!(min_pair_sum(input), 8);
    }
}
