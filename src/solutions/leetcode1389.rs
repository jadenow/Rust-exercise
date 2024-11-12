#[must_use]
pub fn solution(nums: Vec<i32>, index: Vec<usize>) -> Vec<i32> {
    let capacity = nums.len();
    let pairs: Vec<(i32, usize)> = nums.into_iter().zip(index).collect();

    pairs
        .into_iter()
        .fold(Vec::with_capacity(capacity), |mut result, (num, idx)| {
            result.insert(idx, num);
            result
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1389_case1() {
        let nums: Vec<i32> = vec![0, 1, 2, 3, 4];
        let index: Vec<usize> = vec![0, 1, 2, 2, 1];
        let desired: Vec<i32> = vec![0, 4, 1, 3, 2];

        assert_eq!(solution(nums, index), desired);
    }
}
