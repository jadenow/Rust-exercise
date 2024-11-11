use std::convert::TryInto;
#[allow(dead_code)]
fn solution(num: i32) -> i32 {
    println!("2160. Minimum Sum of Four Digit Number After Splitting Digits");

    let mut digits: Vec<i32> = num
        .to_string()
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .expect("Input is between '0' and '9'")
                .try_into()
                .expect("Digits fits in i32")
        })
        .collect();

    digits.sort_unstable();

    let n1 = digits[0] * 10 + digits[3];
    let n2 = digits[1] * 10 + digits[2];
    n1 + n2
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode2160_case1() {
        let num = 4009;
        let desired = 13;
        assert_eq!(solution(num), desired);
    }
}
