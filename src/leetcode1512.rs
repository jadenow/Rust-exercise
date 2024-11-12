use std::collections::HashMap;
#[allow(dead_code)]
fn solution(nums: &[i32]) -> i32 {
    println!("1512. Number of Good Pairs");
    let mut count = HashMap::new();
    let mut ans = 0;

    for &n in nums {
        let cnt = count.entry(n).or_insert(0);
        if *cnt >= 1 {
            ans += *cnt;
        }
        *cnt += 1;
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1512_case1() {
        let nums: Vec<i32> = vec![1, 1, 1, 1];
        let desired: i32 = 6;
        assert_eq!(solution(&nums), desired);
    }
}
