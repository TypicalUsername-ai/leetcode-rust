pub fn total_money(n: i32) -> i32 {
    (0..n).map(|i| i / 7 + i.rem_euclid(7) + 1).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(total_money(4), 10)
    }
    #[test]
    fn case2() {
        assert_eq!(total_money(10), 37)
    }
    #[test]
    fn case3() {
        assert_eq!(total_money(20), 96)
    }
}
