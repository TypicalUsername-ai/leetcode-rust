pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let horizontal_unique: Vec<usize> = mat
        .iter()
        .filter_map(|v| {
            let ones: Vec<usize> = v
                .iter()
                .enumerate()
                .filter_map(|(i, v)| if v == &1 { Some(i) } else { None })
                .collect();
            if ones.len() == 1 {
                Some(ones[0])
            } else {
                None
            }
        })
        .collect();
    horizontal_unique
        .iter()
        .filter(|e| {
            (0..mat.len())
                .filter(|v| {
                    dbg!(v, e);
                    dbg!(mat[*v][**e]) == 1
                })
                .count()
                == 1
        })
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        assert_eq!(num_special(input), 1)
    }

    #[test]
    fn case2() {
        let input = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(num_special(input), 3)
    }

    #[test]
    fn fail1() {
        let input = vec![
            vec![0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(num_special(input), 1)
    }
}
