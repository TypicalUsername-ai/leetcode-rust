pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut smoothed_vals = Vec::with_capacity(img.len());
    for (y, v) in img.iter().enumerate() {
        let mut new_row = Vec::with_capacity(v.len());
        for (x, i) in v.iter().enumerate() {
            // inner to ger surrounding
            let surrounds: Vec<i32> = ((y as i32 - 1).try_into().unwrap_or_default()..=y + 1)
                .filter_map(|ny| img.get(ny))
                .flat_map(|xs| {
                    ((x as i32 - 1).try_into().unwrap_or_default()..=x + 1).map(|nx| xs.get(nx))
                })
                .flatten()
                .cloned()
                .collect();
            new_row.push(surrounds.iter().sum::<i32>() / surrounds.len() as i32)
        }
        smoothed_vals.push(new_row);
    }
    smoothed_vals
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let image = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let smoothed = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];

        assert_eq!(image_smoother(image), smoothed)
    }
    #[test]
    fn case2() {
        let image = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
        let smoothed = vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137],
        ];

        assert_eq!(image_smoother(image), smoothed)
    }
}
