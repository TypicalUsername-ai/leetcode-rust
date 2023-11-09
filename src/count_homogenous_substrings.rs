fn count_homogenous(s: String) -> i32 {
    let mut total: usize = 1;
    let mut streak = 1;
    let mut cs = s.chars();
    let mut prev = cs.next().unwrap();
    for c in cs {
        if c == prev {
            streak += 1;
        } else {
            streak = 1;
        }
        prev = c;
        total += streak;
    }
    total.rem_euclid(10_usize.pow(9) + 7).try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = "abbcccaa".into();

        assert_eq!(count_homogenous(input), 13);
    }

    #[test]
    fn case2() {
        let input = "xy".into();

        assert_eq!(count_homogenous(input), 2);
    }

    #[test]
    fn case3() {
        let input = "zzzzz".into();

        assert_eq!(count_homogenous(input), 15);
    }

    /// first fail
    /// just needed to use pow()
    #[test]
    fn case4() {
        let input = "vvvvvvllll".into();

        assert_eq!(count_homogenous(input), 31);
    }
}
