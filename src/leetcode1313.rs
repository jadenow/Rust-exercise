use std::convert::TryInto;
#[allow(dead_code)]
fn solution(nums: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for pair in nums.chunks(2) {
        result.extend(
            std::iter::repeat(pair[1])
                .take(pair[0].try_into().expect("repeated count is non-negative")),
        );
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1313_case1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let desired: Vec<i32> = vec![2, 4, 4, 4];

        assert_eq!(solution(&nums), desired);
    }
}
