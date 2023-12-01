pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    if word1.iter().fold(0, |acc, a| acc + a.len()) != word2.iter().fold(0, |acc, a| acc + a.len())
    {
        return false;
    }
    word1
        .iter()
        .flat_map(|a| a.chars())
        .zip(word2.iter().flat_map(|a| a.chars()))
        .all(|(a, b)| a == b)
}

#[cfg(test)]
mod test {
    use std::arch::x86_64::_mm256_insertf128_pd;

    use super::*;

    #[test]
    fn case1() {
        let in1 = vec!["ab".to_string(), "c".to_string()];
        let in2 = vec!["a".to_string(), "bc".to_string()];
        assert!(array_strings_are_equal(in1, in2));
    }

    #[test]
    fn case2() {
        let in1 = vec!["a".to_string(), "cb".to_string()];
        let in2 = vec!["ab".to_string(), "c".to_string()];
        assert!(!array_strings_are_equal(in1, in2));
    }

    #[test]
    fn case3() {
        let in1 = vec!["abc".to_string(), "d".to_string(), "defg".to_string()];
        let in2 = vec!["abcddefg".to_string()];
        assert!(array_strings_are_equal(in1, in2));
    }
}
