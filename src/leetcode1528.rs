use std::convert::TryInto;
#[allow(dead_code)]
fn solution(s: &str, indices: &[i32]) -> String {
    s.chars()
        .enumerate()
        .fold(vec![' '; s.len()], |mut shuffled, (idx, val)| {
            let index: usize = indices[idx]
                .try_into()
                .expect("index is non-negative value");
            shuffled[index] = val;
            shuffled
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1528_case1() {
        let s: String = String::from("codeleet");
        let indices: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2, 3];
        let desired: String = String::from("leetcode");

        assert_eq!(solution(&s, &indices), desired);
    }
}
