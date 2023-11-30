fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    // fancy way to write an foreach loop witch can collected int an array
    (0..l.len())
        .map(|i| {
            // getting the start and end of the subarray
            let left = l[i] as usize;
            let right = r[i] as usize;

            // get the subarray and sort it
            // using unstable because its faster and we dont need to preserve the old order
            let mut sub_array: Vec<i32> = nums[left..=right].to_vec();
            sub_array.sort_unstable();

            // calculating the distance between two points
            // Save: because `l[i]` < `r[i]`
            let dist = sub_array[1] - sub_array[0];

            sub_array.windows(2).all(|win| win[1] - win[0] == dist)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 6, 5, 9, 3, 7];
        let l = vec![0, 0, 2];
        let r = vec![2, 3, 5];
        assert_eq!(
            check_arithmetic_subarrays(nums, l, r),
            vec![true, false, true]
        );
    }

    #[test]
    fn case2() {
        let nums = vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10];
        let l = vec![0, 1, 6, 4, 8, 7];
        let r = vec![4, 4, 9, 7, 9, 10];
        assert_eq!(
            check_arithmetic_subarrays(nums, l, r),
            vec![false, true, false, false, true, true]
        );
    }
}

// time limit exceeded
// let mut results = vec![];
// for (s, e) in l.into_iter().zip(r.into_iter()) {
//     let mut slice = (&nums[s as usize..=e as usize]).to_owned();
//     slice.sort_unstable();
//     dbg!(&slice);
//     results.push(
//         slice
//             .windows(3)
//             .all(|arr| arr[0] - arr[1] == arr[1] - arr[2]),
//     );
// }
// results
