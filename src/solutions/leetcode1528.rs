#[must_use]
pub fn solution(s: &str, indices: Vec<usize>) -> String {
    let mut pair: Vec<(usize, char)> = indices.into_iter().zip(s.chars()).collect();
    pair.sort_unstable_by_key(|&(idx, _)| idx);

    pair.into_iter().map(|(_, ch)| ch).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1528_case1() {
        let s: String = String::from("codeleet");
        let indices: Vec<usize> = vec![4, 5, 6, 7, 0, 1, 2, 3];
        let desired: String = String::from("leetcode");

        assert_eq!(solution(&s, indices), desired);
    }
}
