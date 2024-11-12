#[must_use]
pub fn solution(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    std::iter::once(first)
        .chain(encoded.into_iter().scan(first, |now, val| {
            *now ^= val;
            Some(*now)
        }))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1720_case1() {
        let encoded: Vec<i32> = vec![1, 2, 3];
        let first: i32 = 1;
        let desired: Vec<i32> = vec![1, 0, 2, 1];

        assert_eq!(solution(encoded, first), desired);
    }
}
