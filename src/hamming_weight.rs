#[allow(non_snake_case)]
pub fn hammingWeight(n: u32) -> i32 {
    format!("{n:b}")
        .chars()
        .filter(|c| *c == '1')
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(hammingWeight(11), 3);
    }
    #[test]
    fn case2() {
        assert_eq!(hammingWeight(128), 1);
    }
    #[test]
    fn case3() {
        assert_eq!(hammingWeight(4294967293), 31);
    }
}
