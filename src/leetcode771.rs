fn solution(jewels: String, stones: String) -> i32 {
    stones.chars().filter(|&s| jewels.contains(s)).count() as i32
}

mod tests {
    use super::*;

    #[test]
    fn leetcode771_case1() {
        let jewels: String = String::from("aA");
        let stones: String = String::from("aAAbbbb");
        let desired: i32 = 3;

        assert_eq!(solution(jewels, stones), desired);
    }
}
