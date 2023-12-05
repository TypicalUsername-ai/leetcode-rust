pub fn largest_good_integer(num: String) -> String {
    num.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .filter(|a| a[0] == a[1] && a[1] == a[2])
        .max()
        .unwrap_or(&[])
        .iter()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(largest_good_integer("6777133339".to_string()), "777")
    }

    #[test]
    fn case2() {
        assert_eq!(largest_good_integer("2300019".to_string()), "000")
    }

    #[test]
    fn case3() {
        assert_eq!(largest_good_integer("42352338".to_string()), "")
    }
}
