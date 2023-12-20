pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut iter = prices.iter();
    let mut lowest = [iter.next().unwrap(), iter.next().unwrap()];
    if lowest[1] < lowest[0] {
        lowest = [lowest[1], lowest[0]];
    }
    // lowest.sort_unstable();
    for p in iter {
        if p < lowest[0] {
            lowest[1] = lowest[0];
            lowest[0] = p;
        } else if p < lowest[1] {
            lowest[1] = p;
        } else {
            continue;
        };
    }

    let diff = money - (lowest[1] + lowest[0]);
    if diff >= 0 {
        diff
    } else {
        money
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(buy_choco(vec![1, 2, 2], 3), 0)
    }

    #[test]
    fn case2() {
        assert_eq!(buy_choco(vec![3, 2, 3], 3), 3)
    }
}
