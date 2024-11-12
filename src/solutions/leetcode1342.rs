#[must_use]
pub fn solution(num: i32) -> usize {
    std::iter::successors(Some(num), |&current| {
        if current > 0 {
            Some(match current % 2 {
                0 => current / 2,
                _ => current - 1,
            })
        } else {
            None
        }
    })
    .count()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1342_case1() {
        let num: i32 = 14;
        let desired: usize = 6;
        assert_eq!(solution(num), desired);
    }
}
