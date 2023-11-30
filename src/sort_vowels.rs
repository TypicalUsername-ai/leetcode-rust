use std::collections::HashMap;

pub fn sort_vowels(s: String) -> String {
    let wovels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
        .iter()
        .map(|c| (*c, *c as u8))
        .collect::<HashMap<char, u8>>();

    let mut ints = s
        .chars()
        .filter_map(|c| wovels.get(&c))
        .collect::<Vec<&u8>>();
    ints.sort();

    let mut it = ints.iter().map(|i| **i as char);

    s.chars()
        .map(|c| {
            if wovels.get(&c).is_some() {
                (it.next()).unwrap()
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(sort_vowels("lEetcOde".into()), "lEOtcede".to_string());
    }

    #[test]
    fn case2() {
        assert_eq!(sort_vowels("lYmpH".into()), "lYmpH".to_string());
    }
}
