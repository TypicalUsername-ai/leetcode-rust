pub fn convert(s: String, num_rows: i32) -> String {
    if s.len() <= num_rows as usize || num_rows == 1 {
        return s;
    };
    // o to num_rows-1 -> num_rows-2 to 1
    // single iter len is : num_rows + num_rows -2
    let iter_len = 2 * num_rows as usize - 2;

    let mut weighted: Vec<(char, usize)> = s
        .chars()
        .enumerate()
        .map(|c| {
            let rem_iter = c.0.rem_euclid(iter_len);
            let rem_len = c.0.rem_euclid(num_rows as usize);
            if rem_iter >= num_rows as usize {
                (c.1, iter_len - rem_iter)
            } else {
                (c.1, rem_iter)
            }
        })
        .collect();

    weighted.sort_by_key(|k| k.1);
    weighted.iter().map(|a| a.0).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(convert("PAYPALISHIRING".into(), 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn case2() {
        assert_eq!(convert("PAYPALISHIRING".into(), 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn case3() {
        assert_eq!(convert("A".into(), 1), "A");
    }
}
