fn solution(mut n: i32) -> i32 {
    let mut cnt = 0;
    while n > 1 {
        n = match n % 2 {
            0 => {
                cnt += n / 2;
                n / 2
            }
            _ => {
                cnt += (n - 1) / 2;
                (n - 1) / 2 + 1
            }
        };
    }
    cnt
}

mod tests {
    use super::*;

    #[test]
    fn leetcode1688_case1() {
        let n: i32 = 14;
        let desired: i32 = 13;

        assert_eq!(solution(n), desired);
    }
}