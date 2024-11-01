fn solution(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .scan(0, |sum, &n| {
            *sum += n;
            Some(*sum)
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn leetcode1480_case1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let desired: Vec<i32> = vec![1, 3, 6, 10];

        assert_eq!(solution(nums), desired);
    }
}
