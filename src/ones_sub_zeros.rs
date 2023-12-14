pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rows = vec![0; grid.len()];
    let mut cols = vec![0; grid[0].len()];
    for (row, vec) in grid.iter().enumerate() {
        for (col, i) in vec.iter().enumerate() {
            let ic = if i == &1 { 1 } else { -1 };
            rows[row] += ic;
            cols[col] += ic;
        }
    }
    grid.iter()
        .enumerate()
        .map(|(r, v)| {
            v.iter()
                .enumerate()
                .map(|(c, _)| rows[r] + cols[c])
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let output = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        assert_eq!(ones_minus_zeros(input), output)
    }
    #[test]
    fn case2() {
        let input = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let output = vec![vec![5, 5, 5], vec![5, 5, 5]];
        assert_eq!(ones_minus_zeros(input), output)
    }
}
