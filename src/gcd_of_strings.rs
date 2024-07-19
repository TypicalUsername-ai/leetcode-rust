pub fn gcd_of_strings(str1: String, str2: String) -> String {
    for i in (1..=str1.len()).rev() {
        if (str1.len().rem_euclid(i) != 0 && str2.len().rem_euclid(i) != 0) {
            continue;
        }
        let mut cs = str1.as_bytes().rchunks(i);
        let base = cs.next().unwrap();
        if cs.chain(str2.as_bytes().rchunks(i)).all(|a| a == base) {
            return String::from_utf8(base.to_vec()).unwrap();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let str1 = "ABCABC";
        let str2 = "ABC";
        let ans = "ABC";
        assert_eq!(gcd_of_strings(str1.into(), str2.into()), ans);
    }

    #[test]
    fn case_2() {
        let str1 = "ABAB";
        let str2 = "AB";
        let ans = "AB";
        assert_eq!(gcd_of_strings(str1.into(), str2.into()), ans);
    }

    #[test]
    fn no_match_case() {
        let str1 = "LEET";
        let str2 = "CODE";
        let ans = "";
        assert_eq!(gcd_of_strings(str1.into(), str2.into()), ans);
    }

    #[test]
    fn close_no_match() {
        let str1 = "ABABD";
        let str2 = "AB";
        let ans = "";
        assert_eq!(gcd_of_strings(str1.into(), str2.into()), ans);
    }

    #[test]
    fn fail_1() {
        let str1 = "ABABABAB";
        let str2 = "ABAB";
        let ans = "ABAB";
        assert_eq!(gcd_of_strings(str1.into(), str2.into()), ans);
    }
}
