#[must_use]
pub fn solution(nums: &[i32], n: usize) -> Vec<i32> {
    println!("1470. Shuffle the Array");

    let (first_half, second_half) = nums.split_at(n);

    first_half
        .iter()
        .zip(second_half.iter())
        .flat_map(|(&x, &y)| vec![x, y])
        .collect()
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
