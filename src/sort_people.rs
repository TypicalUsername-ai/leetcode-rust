pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut s: Vec<(String, u16)> = names
        .into_iter()
        .zip(heights.into_iter().map(|a| a as u16))
        .collect();
    s.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    s.into_iter().map(|a| a.0).collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case1() {
        let names = ["Mary", "John", "Emma"].map(|a| a.to_string()).to_vec();
        let heights = vec![180, 165, 170];
        let ans = ["Mary", "Emma", "John"].map(|a| a.to_string()).to_vec();
        assert_eq!(sort_people(names, heights), ans)
    }

    #[test]
    fn case2() {
        let names = ["Alice", "Bob", "Bob"].map(|a| a.to_string()).to_vec();
        let heights = vec![155, 185, 150];
        let ans = ["Bob", "Alice", "Bob"].map(|a| a.to_string()).to_vec();
        assert_eq!(sort_people(names, heights), ans)
    }
}
