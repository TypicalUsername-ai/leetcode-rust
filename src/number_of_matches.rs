pub fn number_of_matches(n: i32) -> i32 {
    if n <= 2 {
        return n - 1;
    }
    if n.rem_euclid(2) == 0 {
        n / 2 + number_of_matches(n / 2)
    } else {
        (n - 1) / 2 + number_of_matches((n - 1) / 2 + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case0() {
        assert_eq!(number_of_matches(1), 0)
    }

    #[test]
    fn case1() {
        assert_eq!(number_of_matches(7), 6)
    }

    #[test]
    fn case2() {
        assert_eq!(number_of_matches(14), 13)
    }

    #[test]
    fn stress_case() {
        assert_eq!(number_of_matches(200), 199)
    }
}
