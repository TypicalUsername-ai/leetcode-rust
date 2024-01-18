use std::collections::HashSet;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut v = vec![HashSet::new()];
    for n in nums {
        let mut i = 0;
        while !v[i].insert(n) {
            i += 1;
            if v.len() <= i {
                v.push(HashSet::new());
            }
        }
    }
    v.into_iter().map(|a| a.into_iter().collect()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let inp = vec![1,3,4,1,2,3,1];
        let outs = vec![vec![1,3,4,2],vec![1,3],vec![1]];
        assert_eq!(find_matrix(inp).sort_unstable(), outs.clone().sort_unstable())
    }
    #[test]
    fn case2() {
        let inp = vec![1,2,3,4];
        let outs = vec![vec![1,2,3,4]];
        assert_eq!(find_matrix(inp).sort_unstable(), outs.clone().sort_unstable())
    }
}
