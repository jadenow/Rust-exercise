fn solution(operations: Vec<String>) -> i32 {
    operations.iter().filter(|&op| op.contains('+')).count() as i32
        - operations.iter().filter(|&op| op.contains('-')).count() as i32
}

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

        assert_eq!(solution(operations), desired);
    }
}
