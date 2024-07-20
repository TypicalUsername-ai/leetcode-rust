/// Find any matrix of non-negative integers of size rowSum.length x colSum.length
/// that satisfies the rowSum and colSum requirements.
pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    //create a zero matrix
    let mut matrix = vec![vec![0; col_sum.len()]; row_sum.len()];
    for (row, sum) in matrix.iter_mut().zip(row_sum.iter()) {
        row[0] = *sum;
    }

    for col in 0..col_sum.len() - 1 {
        let mut run_col = 0;
        let target = col_sum[col];
        for row in 0..row_sum.len() {
            if matrix[row][col] + run_col <= target {
                run_col += matrix[row][col];
            } else if run_col < target {
                matrix[row][col + 1] += matrix[row][col] - (target - run_col);
                matrix[row][col] = target - run_col;
                run_col += target - run_col;
            } else {
                matrix[row][col + 1] += matrix[row][col];
                matrix[row][col] = 0;
            }
        }
    }

    matrix
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn basic_case() {
        let row_sum = vec![1];
        let col_sum = vec![1];
        let ans = vec![vec![1]];
        assert_eq!(restore_matrix(row_sum, col_sum), ans)
    }

    #[test]
    fn simple_case() {
        let row_sum = vec![3, 8];
        let col_sum = vec![4, 7];
        let ans = restore_matrix(row_sum.clone(), col_sum.clone());
        // check rows
        assert!(ans
            .iter()
            .zip(row_sum.into_iter())
            .all(|(m, s)| m.iter().sum::<i32>() == s));
        assert!((0..col_sum.len())
            .map(|i| ans.iter().map(|a| a[i]).sum::<i32>())
            .zip(col_sum.iter())
            .all(|(a, b)| a == *b))
    }

    #[test]
    fn advanced_case() {
        let row_sum = vec![5, 7, 10];
        let col_sum = vec![8, 6, 8];
        let ans = restore_matrix(row_sum.clone(), col_sum.clone());
        // check rows
        assert!(ans
            .iter()
            .zip(row_sum.into_iter())
            .all(|(m, s)| m.iter().sum::<i32>() == s));
        assert!((0..col_sum.len())
            .map(|i| ans.iter().map(|a| a[i]).sum::<i32>())
            .zip(col_sum.iter())
            .all(|(a, b)| a == *b))
    }

    #[test]
    fn uneven_case() {
        let row_sum = vec![14, 9];
        let col_sum = vec![6, 9, 8];
        let ans = restore_matrix(row_sum.clone(), col_sum.clone());
        // check rows
        assert!(ans
            .iter()
            .zip(row_sum.into_iter())
            .all(|(m, s)| m.iter().sum::<i32>() == s));
        assert!((0..col_sum.len())
            .map(|i| ans.iter().map(|a| a[i]).sum::<i32>())
            .zip(col_sum.iter())
            .all(|(a, b)| a == *b))
    }

    #[test]
    fn uneven2_case() {
        let row_sum = vec![30; 2];
        let col_sum = vec![15; 4];
        let ans = restore_matrix(row_sum.clone(), col_sum.clone());
        // check rows
        assert!(ans
            .iter()
            .zip(row_sum.into_iter())
            .all(|(m, s)| m.iter().sum::<i32>() == s));
        assert!((0..col_sum.len())
            .map(|i| ans.iter().map(|a| a[i]).sum::<i32>())
            .zip(col_sum.iter())
            .all(|(a, b)| a == *b))
    }

    #[test]
    fn zeroes_case() {
        let row_sum = vec![4, 12, 10, 1, 0];
        let col_sum = vec![1, 0, 3, 16, 7];
        let ans = restore_matrix(row_sum.clone(), col_sum.clone());
        // check rows
        assert!(ans
            .iter()
            .zip(row_sum.into_iter())
            .all(|(m, s)| m.iter().sum::<i32>() == s));
        assert!((0..col_sum.len())
            .map(|i| ans.iter().map(|a| a[i]).sum::<i32>())
            .zip(col_sum.iter())
            .all(|(a, b)| a == *b))
    }

    #[test]
    fn stress_case() {
        let row_sum = vec![100000000; 500];
        let col_sum = vec![100000000; 500];
        let ans = restore_matrix(row_sum.clone(), col_sum.clone());
        // check rows
        assert!(ans
            .iter()
            .zip(row_sum.into_iter())
            .all(|(m, s)| m.iter().sum::<i32>() == s));
        assert!((0..col_sum.len())
            .map(|i| ans.iter().map(|a| a[i]).sum::<i32>())
            .zip(col_sum.iter())
            .all(|(a, b)| a == *b))
    }
}
