pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut sorted: Vec<i32> = points.iter().map(|p| p[0]).collect();
    sorted.sort_unstable();
    sorted.leak().windows(2).map(|v| v[1] - v[0]).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
        assert_eq!(max_width_of_vertical_area(input), 1)
    }

    #[test]
    fn case2() {
        let input = vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8],
        ];
        assert_eq!(max_width_of_vertical_area(input), 3)
    }
}
