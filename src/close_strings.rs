use std::collections::HashMap;

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    if word1.len() != word2.len() {
        return false;
    }
    for c in word1.chars() {
        map1.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    for c in word2.chars() {
        map2.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut c1 = map1.keys().collect::<Vec<&char>>();
    let mut c2 = map2.keys().collect::<Vec<&char>>();
    c1.sort_unstable();
    c2.sort_unstable();
    let mut i1 = map1.values().collect::<Vec<&i32>>();
    let mut i2 = map2.values().collect::<Vec<&i32>>();
    i1.sort_unstable();
    i2.sort_unstable();
    c1 == c2 && i1 == i2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(close_strings("abc".to_string(), "bca".to_string()), true)
    }

    #[test]
    fn case2() {
        assert_eq!(close_strings("a".to_string(), "aa".to_string()), false)
    }

    #[test]
    fn case3() {
        assert_eq!(
            close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        )
    }

    #[test]
    fn case4() {
        assert_eq!(
            close_strings("cabbba".to_string(), "aabbss".to_string()),
            false
        )
    }
}
