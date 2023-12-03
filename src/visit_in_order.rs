pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let distances = points
        .windows(2)
        .map(|a| [(a[0][0] - a[1][0]).abs(), (a[0][1] - a[1][1]).abs()]);
    dbg!(&distances);
    distances.fold(0, |time, d| {
        if d[0] > d[1] {
            time + d[1] + (d[0] - d[1])
        } else {
            time + d[0] + (d[1] - d[0])
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![[1, 1].into(), [3, 4].into(), [-1, 0].into()];

        assert_eq!(min_time_to_visit_all_points(input), 7);
    }
}
