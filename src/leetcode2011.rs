use std::convert::TryFrom;
#[allow(dead_code)]
fn solution(operations: &[String]) -> i32 {
    let increment_count = i32::try_from(operations.iter().filter(|&op| op.contains('+')).count())
        .expect("The count is within the i32 range");
    let decrement_count = i32::try_from(operations.iter().filter(|&op| op.contains('-')).count())
        .expect("The count is within the i32 range");

    increment_count - decrement_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode2011_case1() {
        let operations: Vec<String> = vec![
            String::from("--X"),
            String::from("X++"),
            String::from("X++"),
        ];
        let desired: i32 = 1;

        assert_eq!(solution(&operations), desired);
    }
}
