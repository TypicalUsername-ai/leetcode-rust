use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map = HashMap::new();
    for c in s.chars() {
        s_map.insert(c, s_map.get(&c).unwrap_or(&0) + 1);
    }
    for c in t.chars() {
        s_map.insert(c, s_map.get(&c).unwrap_or(&0) - 1);
    }
    s_map.values().all(|v| v == &0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert!(is_anagram(s, t))
    }

    #[test]
    fn case2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert_eq!(is_anagram(s, t), false)
    }

    #[test]
    fn case3() {
        let s = "rat".to_string();
        let t = "ratt".to_string();
        assert_eq!(is_anagram(s, t), false)
    }
}
