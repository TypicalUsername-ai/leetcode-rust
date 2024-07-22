pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extensive_case() {
        let s = "  a  good case  ";
        let ans = "case good a";
        assert_eq!(reverse_words(s.to_string()), ans)
    }
}
