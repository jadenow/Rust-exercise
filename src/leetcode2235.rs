#[allow(dead_code)]
fn solution(num1: i32, num2: i32) -> i32 {
    println!("2235. Add Two Integers");

    num1 + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode2235_case1() {
        let num1: i32 = 5;
        let num2: i32 = 7;
        let desired = 12;
        assert_eq!(solution(num1, num2), desired);
    }
}
