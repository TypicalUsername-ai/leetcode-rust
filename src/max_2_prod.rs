pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut iter = nums.iter();
    let (mut n1, mut n2) = (0, 0);
    for n in iter {
        if n > &n2 {
            n1 = n2;
            n2 = *n;
        } else if n > &n1 {
            n1 = *n;
        }
    }
    (n1 - 1) * (n2 - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12)
    }

    #[test]
    fn case2() {
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16)
    }

    #[test]
    fn case3() {
        assert_eq!(max_product(vec![3, 7]), 12)
    }
}
