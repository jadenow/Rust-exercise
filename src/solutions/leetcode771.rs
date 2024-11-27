#[must_use]
pub fn solution(jewels: &str, stones: &str) -> usize {
    stones.chars().filter(|s| jewels.contains(*s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leetcode771_case1() {
        let jewels: String = String::from("aA");
        let stones: String = String::from("aAAbbbb");
        let desired: usize = 3;

        assert_eq!(solution(&jewels, &stones), desired);
    }
}
