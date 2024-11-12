#[must_use]
pub fn solution(nums: &[usize]) -> Vec<usize> {
    nums.chunks(2)
        .filter_map(|pair| match (pair.first(), pair.get(1)) {
            (Some(&count), Some(&value)) => Some(std::iter::repeat(value).take(count)),
            _ => None,
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1313_case1() {
        let nums: Vec<usize> = vec![1, 2, 3, 4];
        let desired: Vec<usize> = vec![2, 4, 4, 4];

        assert_eq!(solution(&nums), desired);
    }
}
