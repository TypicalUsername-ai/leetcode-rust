use std::cell::RefCell;

fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut vec = nums
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, e)| ([i + j, j], e))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    vec.sort_unstable();

    let vec = vec.into_iter().map(|(_, e)| e).collect::<Vec<_>>();

    vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let nums = [[1, 2, 3].into(), [4, 5, 6].into(), [7, 8, 9].into()].into();
        assert_eq!(find_diagonal_order(nums), vec![1, 4, 2, 7, 5, 3, 8, 6, 9]);
    }

    #[test]
    fn case2() {
        let nums = [
            [1, 2, 3, 4, 5].into(),
            [6, 7].into(),
            [8].into(),
            [9, 10, 11].into(),
            [12, 13, 14, 15, 16].into(),
        ]
        .into();
        assert_eq!(
            find_diagonal_order(nums),
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        );
    }
}

// Exceeds time limit
// let max_size = nums
//     .iter()
//     .map(|a| a.len())
//     .reduce(|acc, a| acc + a)
//     .unwrap();
// let mut iters: Vec<_> = nums
//     .clone()
//     .into_iter()
//     .map(|a| RefCell::new(a.into_iter()))
//     .collect();

// let mut arr = vec![];

// for i in 0..max_size {
//     for (ind, j) in iters.iter().take(i + 1).rev().enumerate() {
//         match (*j.borrow_mut()).next() {
//             Some(a) => {
//                 arr.push(a);
//             }
//             None => {
//                 continue;
//             }
//         }
//     }
// }
// arr
