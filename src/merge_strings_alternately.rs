pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut i1 = word1.chars();
    let mut i2 = word2.chars();
    let mut s = vec![];
    let mut flag = true;
    while flag {
        flag = false;
        if let Some(c) = i1.next() {
            flag = true;
            s.push(c);
        }
        if let Some(c) = i2.next() {
            flag = true;
            s.push(c);
        }
    }
    s.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let s1 = "abc";
        let s2 = "pqr";
        let ans = "apbqcr";
        assert_eq!(merge_alternately(s1.into(), s2.into()), ans);
    }
    #[test]
    fn case2() {
        let s1 = "abc";
        let s2 = "pqrs";
        let ans = "apbqcrs";
        assert_eq!(merge_alternately(s1.into(), s2.into()), ans);
    }
    #[test]
    fn case3() {
        let s1 = "abcd";
        let s2 = "pqr";
        let ans = "apbqcrd";
        assert_eq!(merge_alternately(s1.into(), s2.into()), ans);
    }
}
