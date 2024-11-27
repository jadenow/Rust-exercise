#[must_use]
pub fn solution(nums: &[i32]) -> Vec<usize> {
    println!("1365. How Many Numbers Are Smaller Than the Current Number");

    nums.iter()
        .map(|&n| nums.iter().filter(|&&m| n > m).count())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1365_case1() {
        let nums: Vec<i32> = vec![6, 5, 4, 8];
        let desired: Vec<usize> = vec![2, 1, 0, 3];
        assert_eq!(solution(&nums), desired);
    }
}
