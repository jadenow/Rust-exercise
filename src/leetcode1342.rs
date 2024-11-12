#[allow(dead_code)]
fn solution(mut num: i32) -> i32 {
    let mut cnt = 0;
    while num > 0 {
        cnt += 1;
        num = match num % 2 {
            0 => num / 2,
            _ => num - 1,
        };
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1342_case1() {
        let num: i32 = 14;
        let desired: i32 = 6;
        assert_eq!(solution(num), desired);
    }
}
