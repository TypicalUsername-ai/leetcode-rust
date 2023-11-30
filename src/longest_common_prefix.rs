fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].to_owned();
    }

    let mut carrs = strs.into_iter();
    let mut base = carrs.next().unwrap();

    for cmp in carrs {
        let mut new_base = vec![];

        for (c1, c2) in base.chars().zip(cmp.chars()) {
            if (c1 == c2) {
                new_base.push(c2)
            } else {
                break;
            }
        }
        base = new_base.iter().collect();
    }

    base
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        assert_eq!(longest_common_prefix(input), "fl".to_string());
    }

    #[test]
    fn case2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

        assert_eq!(longest_common_prefix(input), "".to_string());
    }
}
