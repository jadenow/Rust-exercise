#[must_use]
pub fn solution(nums: &[usize]) -> Vec<usize> {
    println!("1920. Build Array from Permutation");

    nums.iter().filter_map(|&i| nums.get(i)).copied().collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1920_case1() {
        let nums: Vec<usize> = vec![0, 2, 1, 5, 3, 4];
        let desired = [0, 1, 2, 4, 5, 3];
        assert_eq!(solution(&nums), desired);
    }
}
