pub fn is_palindrome(x: i32) -> bool {
    let str = x.to_string();

    for (front, back) in str.chars().zip(str.chars().rev()) {
        if front != back {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn case2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn case3() {
        assert_eq!(is_palindrome(10), false);
    }
}
