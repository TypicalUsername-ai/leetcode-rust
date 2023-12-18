pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    let len = sorted.len();
    (sorted[len - 1] * sorted[len - 2]) - (sorted[0] * sorted[1])
}

#[cfg(test)]
mod test {
    use std::cmp::max_by_key;

    use super::*;

    #[test]
    fn case1() {
        let input = vec![5, 6, 2, 7, 4];
        assert_eq!(max_product_difference(input), 34)
    }

    #[test]
    fn case2() {
        let input = vec![4, 2, 5, 9, 7, 4, 8];
        assert_eq!(max_product_difference(input), 64)
    }
}
