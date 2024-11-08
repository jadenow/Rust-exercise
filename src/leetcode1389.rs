fn solution(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());

    for (num, idx) in nums.into_iter().zip(index.into_iter()) {
        result.insert(idx as usize, num);
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn leetcode1389_case1() {
        let nums: Vec<i32> = vec![0, 1, 2, 3, 4];
        let index: Vec<i32> = vec![0, 1, 2, 2, 1];
        let desired: Vec<i32> = vec![0, 4, 1, 3, 2];

        assert_eq!(solution(nums, index), desired);
    }
}
