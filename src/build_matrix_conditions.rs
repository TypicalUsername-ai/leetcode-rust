use std::collections::{HashMap, HashSet};

///You are given a positive integer k. You are also given
///a 2D integer array rowConditions of size n where rowConditions[i] = [above_i, below_i], and
///a 2D integer array colConditions of size m where colConditions[i] = [left_i, right_i].
///The two arrays contain integers from 1 to k.
///You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.
///The matrix should also satisfy the following conditions:
///The number above i should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
///The number left i should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.
///Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.
pub fn build_matrix(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    col_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    debug_assert!(k >= 2);
    let to_insert = (1i32..=k).collect::<Vec<_>>();

    // create a set of all values that should be above all given values
    let mut rows_above = HashMap::<i32, HashSet<i32>>::from_iter(
        to_insert.clone().iter().map(|a| (*a, HashSet::new())),
    );
    for cond in row_conditions.iter() {
        let mut alr = rows_above.get(&cond[0]).unwrap().clone();
        alr.insert(cond[0]);
        if alr.contains(&cond[1]) {
            return vec![];
        }
        rows_above.get_mut(&cond[1]).unwrap().extend(alr);
    }

    // create a set of all values that should be left of all given values
    let mut cols_left = HashMap::<i32, HashSet<i32>>::from_iter(
        to_insert.clone().iter().map(|a| (*a, HashSet::new())),
    );
    for cond in col_conditions.iter() {
        let mut alr = cols_left.get(&cond[0]).unwrap().clone();
        alr.insert(cond[0]);
        if alr.contains(&cond[1]) {
            return vec![];
        }
        cols_left.get_mut(&cond[1]).unwrap().extend(alr);
    }

    let mut matrix = vec![vec![0; k as usize]; k as usize];
    let mut row_ordered = Vec::with_capacity(k as usize);

    // drain the row set from empty to put it in order
    while !rows_above.is_empty() {
        let to_rm = rows_above.iter().find(|(k, v)| v.is_empty());
        if to_rm.is_none() {
            return vec![];
        }
        let to_rm = to_rm.unwrap().0.to_owned();
        rows_above.remove(&to_rm);
        // eprintln!("REMOVING {}", &to_rm);
        row_ordered.push(to_rm);
        for set in rows_above.values_mut() {
            set.remove(&to_rm);
        }
    }

    let mut col = 0;
    // drain the col set from empty to put it in order
    while !cols_left.is_empty() {
        let to_rm = cols_left.iter().find(|(k, v)| v.is_empty());
        if to_rm.is_none() {
            return vec![];
        }
        let to_rm = to_rm.unwrap().0.to_owned();

        let row = row_ordered
            .iter()
            .enumerate()
            .find_map(|(i, a)| if *a == to_rm { Some(i) } else { None })
            .unwrap();
        cols_left.remove(&to_rm);
        // eprintln!("REMOVING {}", &to_rm);
        // eprintln!("m[{}][{}] = {}", row, col, &to_rm);
        matrix[row][col] = to_rm;
        // eprintln!("{}", matrix[row][col]);
        col += 1;
        for set in cols_left.values_mut() {
            set.remove(&to_rm);
        }
    }
    matrix
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_answer(
        k: i32,
        ans: Vec<Vec<i32>>,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) {
        assert_eq!(ans.len(), k as usize, "Expected {k} rows got {}", ans.len());
        assert_eq!(
            ans[0].len(),
            k as usize,
            "Expected {k} cols got {}",
            ans[0].len()
        );
        let by_row = ans
            .iter()
            .map(|a| a.iter())
            .flatten()
            .filter(|e| **e != 0)
            .collect::<Vec<_>>();
        let by_col = (0..k as usize)
            .map(|i| ans.iter().map(move |a| a[i]))
            .flatten()
            .filter(|e| *e != 0)
            .collect::<Vec<_>>();
        for cond in row_conditions {
            assert!(
                by_row
                    .iter()
                    .take_while(|a| ***a != cond[1])
                    .find(|a| ***a == cond[0])
                    .is_some(),
                "{} not above {} ({cond:?})",
                cond[0],
                cond[1]
            );
        }
        for cond in col_conditions {
            assert!(
                by_col
                    .iter()
                    .take_while(|a| **a != cond[1])
                    .find(|a| **a == cond[0])
                    .is_some(),
                "{} not left of {} ({cond:?})",
                cond[0],
                cond[1]
            )
        }
    }

    fn print_matrix(matrix: &Vec<Vec<i32>>) {
        for a in matrix {
            eprintln!("{a:?}");
        }
    }

    #[test]
    fn basic_case() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![3, 2]];
        let col_conditions = vec![vec![2, 1], vec![3, 2]];
        let ans = build_matrix(k, row_conditions.clone(), col_conditions.clone());
        print_matrix(&ans);
        test_answer(k, ans, row_conditions, col_conditions);
    }

    #[test]
    fn fail_rows() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let col_conditions = vec![vec![2, 1], vec![3, 2]];
        let ans = build_matrix(k, row_conditions.clone(), col_conditions.clone());
        assert!(ans.is_empty(), "Row conditions should be invalid")
    }

    #[test]
    fn fail_cols() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![2, 3]];
        let col_conditions = vec![vec![2, 1], vec![1, 3], vec![3, 2]];
        let ans = build_matrix(k, row_conditions.clone(), col_conditions.clone());
        assert!(ans.is_empty(), "Column conditions should be invalid")
    }

    #[test]
    fn none_to_progress() {
        let k = 258;
        let row_conditions = vec![vec![1, 2], vec![3, 4]];
        let col_conditions = vec![vec![1, 2], vec![3, 4]];
        let ans = build_matrix(k, row_conditions.clone(), col_conditions.clone());
        print_matrix(&ans);
        test_answer(k, ans, row_conditions, col_conditions);
    }

    #[test]
    fn stress_case() {
        let k = 400;
        let row_conditions = (1..k).map(|i| vec![i, i + 1]).collect::<Vec<_>>();
        let col_conditions = (1..k).map(|i| vec![i, i + 1]).collect::<Vec<_>>();
        let ans = build_matrix(k, row_conditions.clone(), col_conditions.clone());
        test_answer(k, ans, row_conditions, col_conditions);
    }
}
