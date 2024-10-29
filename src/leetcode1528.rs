fn solution(s: String, indices: Vec<i32>) -> String {
    s.chars()
        .enumerate()
        .fold(vec![' '; s.len()], |mut shuffled, (idx, val)| {
            shuffled[indices[idx] as usize] = val;
            shuffled
        })
        .into_iter()
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn leetcode1528_case1() {
        let s: String = String::from("codeleet");
        let indices: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2, 3];
        let desired: String = String::from("leetcode");

        assert_eq!(solution(s, indices), desired);
    }
}
