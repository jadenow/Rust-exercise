#[must_use]
pub fn solution(operations: &[&str]) -> i32 {
    operations.iter().fold(
        0,
        |acc, op| {
            if op.contains('+') {
                acc + 1
            } else {
                acc - 1
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode2011_case1() {
        let operations: Vec<&str> = vec!["--X", "X++", "X++"];
        let desired: i32 = 1;

        assert_eq!(solution(&operations), desired);
    }
}
