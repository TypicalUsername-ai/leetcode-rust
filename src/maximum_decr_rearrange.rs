pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
    let mut arr_mut = arr.clone();
    arr_mut.sort_unstable();
    let mut iter = arr_mut.iter_mut();
    *iter.next().unwrap() = 1;
    let mut prev = 1;
    for i in iter {
        if *i == prev || *i == prev + 1 {
            prev = *i;
            continue;
        } else {
            *i = prev + 1;
        }
        prev = *i;
    }
    *arr_mut.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![2, 2, 1, 2, 1];

        let r = maximum_element_after_decrementing_and_rearranging(input);

        assert_eq!(r, 2);
    }

    #[test]
    fn case2() {
        let input = vec![100, 1, 1000];

        let r = maximum_element_after_decrementing_and_rearranging(input);

        assert_eq!(r, 3);
    }

    #[test]
    fn case3() {
        let input = vec![1, 2, 3, 4, 5];

        let r = maximum_element_after_decrementing_and_rearranging(input);

        assert_eq!(r, 5);
    }
}
