use std::collections::HashMap;

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut sum = 0;
    let hmap = chars.chars().fold(HashMap::new(), |mut m, c| {
        if let Some(i) = m.get(&c) {
            m.insert(c, i + 1)
        } else {
            m.insert(c, 1)
        };
        m
    });
    for w in words {
        let w_map = w.chars().fold(HashMap::new(), |mut m, c| {
            if let Some(i) = m.get(&c) {
                m.insert(c, i + 1)
            } else {
                m.insert(c, 1)
            };
            m
        });
        if w_map.iter().all(|(c, i)| {
            if let Some(v) = hmap.get(c) {
                v >= i
            } else {
                false
            }
        }) {
            sum += w.len();
        }
    }
    sum as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![
            "cat".to_string(),
            "bt".to_string(),
            "hat".to_string(),
            "tree".to_string(),
        ];
        let chars = "atach".to_string();

        assert_eq!(count_characters(input, chars), 6);
    }

    #[test]
    fn case2() {
        let input = vec![
            "hello".to_string(),
            "world".to_string(),
            "leetcode".to_string(),
        ];
        let chars = "welldonehoneyr".to_string();

        assert_eq!(count_characters(input, chars), 10);
    }
}
