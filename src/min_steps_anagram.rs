// we know s.len() == t.len()
pub fn min_steps(s: String, t: String) -> i32 {
    let mut map1 = [0i32; 26];
    let mut map2 = [0i32; 26];
    let a = 'a' as usize;

    for (c1, c2) in s.chars().zip(t.chars()) {
        map1[c1 as usize - a] += 1;
        map2[c2 as usize - a] += 1;
    }

    map1.into_iter()
        .zip(map2)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>() as i32
        / 2i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(min_steps("bab".to_string(), "aba".to_string()), 1)
    }

    #[test]
    fn case2() {
        assert_eq!(min_steps("leetcode".to_string(), "practice".to_string()), 5)
    }

    #[test]
    fn case3() {
        assert_eq!(min_steps("anagram".to_string(), "mangaar".to_string()), 0)
    }
}
