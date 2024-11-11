use std::convert::TryInto;
#[allow(dead_code)]
fn solution(nums: &[i32], n: i32) -> Vec<i32> {
    println!("1470. Shuffle the Array");

    let nsize = n.try_into().expect("n is non-negative");

    let mut ans = Vec::new();

    for i in 0..nsize {
        ans.push(nums[i]);
        ans.push(nums[i + nsize]);
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1470_case1() {
        let desired: Vec<i32> = vec![2, 3, 5, 4, 1, 7];
        let n = 3;
        let nums: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
        assert_eq!(solution(&nums, n), desired);
    }
}
