pub fn largest_odd_number(num: String) -> String {
    let ind = num
        .char_indices()
        .fold(0, |acc, e| if e.1 as i32 % 2 == 1 { e.0 + 1 } else { acc });
    num[0..ind].to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(largest_odd_number("52".into()), "5");
    }

    #[test]
    fn case2() {
        assert_eq!(largest_odd_number("4206".into()), "");
    }

    #[test]
    fn case3() {
        assert_eq!(largest_odd_number("35427".into()), "35427");
    }
}
