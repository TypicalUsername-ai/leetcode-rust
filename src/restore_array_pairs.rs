use std::collections::HashMap;

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut answer = Vec::with_capacity(adjacent_pairs.len());
    for v in adjacent_pairs {
        map.entry(v[0]).or_insert(vec![]).push(v[1]);
        map.entry(v[1]).or_insert(vec![]).push(v[0]);
    }

    let mut curr = *map.iter().find(|(_, x)| x.len() == 1).unwrap().0;

    while let Some((_, nexts)) = map.remove_entry(&curr) {
        answer.push(curr);

        if let Some(next) = nexts.iter().find(|x| map.contains_key(x)) {
            curr = *next;
        }
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    // skipping bc random sort order
    #[test]
    #[ignore]
    fn case1() {
        let pairs = vec![[2, 1].into(), [3, 4].into(), [3, 2].into()];

        assert_eq!(restore_array(pairs), vec![4, 3, 2, 1])
    }

    #[test]
    #[ignore]
    fn case2() {
        let pairs = vec![[4, -2].into(), [1, 4].into(), [-3, 1].into()];

        assert_eq!(restore_array(pairs), vec![-3, 1, 4, -2])
    }

    #[test]
    #[ignore]
    fn case3() {
        let pairs = vec![[100000, -100000].into()];

        assert_eq!(restore_array(pairs), vec![-100000, 100000])
    }
}
