use std::fmt::Binary;

fn find_different_binary_string(nums: Vec<String>) -> String {
    let sorted = {
        let mut a: Vec<u16> = nums
            .clone()
            .iter()
            .map(|n| u16::from_str_radix(n, 2).unwrap())
            .collect();
        a.sort_unstable();
        a
    };
    for (i, v) in (0..nums.len() as u16).zip(sorted.into_iter()) {
        dbg!(i, v);
        if i != v {
            return format!("{i:0width$b}", width = nums.len());
        }
    }
    format!("{:0width$b}", nums.len(), width = nums.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let res = find_different_binary_string(vec!["01".into(), "10".into()]);
        assert_eq!(res, "00");
    }

    #[test]
    fn case2() {
        let res = find_different_binary_string(vec!["00".into(), "01".into()]);
        assert_eq!(res, "10");
    }

    #[test]
    fn case3() {
        let res = find_different_binary_string(vec!["111".into(), "011".into(), "001".into()]);
        assert_eq!(res, "000");
    }
}
