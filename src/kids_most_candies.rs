pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().reduce(|a, b| a.max(b)).unwrap();
    candies.iter().map(|a| a + extra_candies >= *max).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let kids = vec![2, 3, 5, 1, 3];
        let candies = 3;
        let ans = vec![true, true, true, false, true];
        assert_eq!(kids_with_candies(kids, candies), ans)
    }

    #[test]
    fn case2() {
        let kids = vec![4, 2, 1, 1, 2];
        let candies = 1;
        let ans = vec![true, false, false, false, false];
        assert_eq!(kids_with_candies(kids, candies), ans)
    }

    #[test]
    fn case3() {
        let kids = vec![12, 1, 12];
        let candies = 10;
        let ans = vec![true, false, true];
        assert_eq!(kids_with_candies(kids, candies), ans)
    }
}
