use std::collections::HashSet;

pub fn is_path_crossing(path: String) -> bool {
    let mut hset = HashSet::from([(0, 0)]);
    let mut loc = (0, 0);
    for d in path.chars() {
        match d {
            'N' => {
                loc = (loc.0, loc.1 + 1);
                if !hset.insert(loc) {
                    return true;
                }
            }
            'E' => {
                loc = (loc.0 + 1, loc.1);
                if !hset.insert(loc) {
                    return true;
                }
            }
            'S' => {
                loc = (loc.0, loc.1 - 1);
                if !hset.insert(loc) {
                    return true;
                }
            }
            'W' => {
                loc = (loc.0 - 1, loc.1);
                if !hset.insert(loc) {
                    return true;
                }
            }
            _ => unreachable!(),
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let path = "NES".to_string();
        assert!(!is_path_crossing(path))
    }

    #[test]
    fn case2() {
        let path = "NESWW".to_string();
        assert!(is_path_crossing(path))
    }
}
