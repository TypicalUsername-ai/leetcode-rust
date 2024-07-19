/// search for lucky numbers in matrix
/// a lucky number is one that is a minimum in its row and maximum in its column
pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut luckies = vec![];
    for row in &matrix {
        let mut min = row[0];
        let mut min_index = 0;
        let mut nums = row.iter().enumerate();
        nums.next();
        // find the minimum in row
        for (i, x) in nums {
            if (x < &min) {
                min = *x;
                min_index = i;
            }
        }
        if matrix.iter().map(|a| a[min_index]).all(|i| i <= min) {
            luckies.push(min);
        }
    }
    luckies
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minimal_case() {
        let matrix = vec![vec![1]];
        let ans = vec![1];
        let mut numbers = lucky_numbers(matrix);
        numbers.sort_unstable();
        assert_eq!(numbers, ans)
    }

    #[test]
    fn case_2x2() {
        let matrix = vec![vec![7, 8], vec![1, 2]];
        let ans = vec![7];
        let mut numbers = lucky_numbers(matrix);
        numbers.sort_unstable();
        assert_eq!(numbers, ans)
    }

    #[test]
    fn case_4x3() {
        let matrix = vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]];
        let ans = vec![12];
        let mut numbers = lucky_numbers(matrix);
        numbers.sort_unstable();
        assert_eq!(numbers, ans)
    }
}
