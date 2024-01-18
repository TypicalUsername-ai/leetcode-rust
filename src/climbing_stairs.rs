pub fn climb_stairs(n: i32) -> i32 {
    let mut v = vec![1, 2, 3];
    while v.len() < n as usize {
        let last = v.len() - 1;
        v.push(v[last] + v[last - 1])
    }
    v[(n - 1) as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(climb_stairs(2), 2)
    }

    #[test]
    fn case2() {
        assert_eq!(climb_stairs(3), 3)
    }

    #[test]
    fn case3() {
        assert_eq!(climb_stairs(5), 8)
    }

    #[test]
    fn finge_case() {
        assert_eq!(climb_stairs(45), 1836311903)
    }
}
