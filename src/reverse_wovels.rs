const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

pub fn reverse_vowels(s: String) -> String {
    let mut stack = vec![];
    let no_vow: Vec<char> = s
        .chars()
        .into_iter()
        .map(|c| {
            if VOWELS.contains(&c) {
                stack.push(c);
                '\0'
            } else {
                c
            }
        })
        .collect();
    no_vow
        .into_iter()
        .map(|c| if c == '\0' { stack.pop().unwrap() } else { c })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let s = "hello";
        let ans = "holle";
        assert_eq!(reverse_vowels(s.to_string()), ans)
    }

    #[test]
    fn case2() {
        let s = "leetcode";
        let ans = "leotcede";
        assert_eq!(reverse_vowels(s.to_string()), ans)
    }
}
