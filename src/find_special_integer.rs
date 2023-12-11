pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let treshold = arr.len() / 4;
    let mut count = 0;
    let mut iter = arr.iter();
    let mut prev = iter.next().unwrap();
    for i in iter {
        if count >= treshold {
            return *prev;
        }
        if i == prev {
            count += 1;
        } else {
            prev = i;
            count = 0;
        }
    }
    *prev
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6)
    }

    #[test]
    fn case2() {
        assert_eq!(find_special_integer(vec![1, 1]), 1)
    }

    #[test]
    fn case3() {
        assert_eq!(find_special_integer(vec![9]), 9)
    }
}
