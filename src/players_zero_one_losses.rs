use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    for game in matches {
        let winner = game[0];
        map.entry(winner).or_insert(0);
        let loser = game[1];
        map.entry(loser).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut v = map.iter().fold(vec![vec![], vec![]], |mut acc, v| {
        if v.1 == &1 {
            acc[1].push(*v.0);
        } else if v.1 == &0 {
            acc[0].push(*v.0);
        }
        acc
    });
    v[0].sort_unstable();
    v[1].sort_unstable();
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let output = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(find_winners(input), output);
    }

    #[test]
    fn case2() {
        let input = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let output = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(find_winners(input), output)
    }
}
