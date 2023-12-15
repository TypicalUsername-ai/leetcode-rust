use std::collections::HashSet;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut starts = HashSet::new();
    let mut dests = HashSet::new();

    for p in paths {
        starts.insert(p[0].clone());
        dests.insert(p[1].clone());
    }
    dests.difference(&starts).next().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()],
        ];
        assert_eq!(dest_city(input), "Sao Paulo")
    }

    #[test]
    fn case2() {
        let input = vec![
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "B".to_string()],
            vec!["C".to_string(), "A".to_string()],
        ];
        assert_eq!(dest_city(input), "A")
    }

    #[test]
    fn case3() {
        let input = vec![vec!["A".to_string(), "Z".to_string()]];
        assert_eq!(dest_city(input), "Z")
    }
}
