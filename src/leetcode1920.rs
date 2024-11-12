#[allow(dead_code)]
fn solution(nums: Vec<i32>) -> Vec<i32> {
    println!("1920. Build Array from Permutation");

    let nsize = nums.len();
    let mut ans: Vec<i32> = Vec::new();

    for i in 0..nsize {
        let now = nums[i] as usize;
        ans.push(nums[now]);
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1920_case1() {
        let nums: Vec<i32> = vec![0, 2, 1, 5, 3, 4];
        let desired = [0, 1, 2, 4, 5, 3];
        assert_eq!(solution(nums), desired);
    }
}
