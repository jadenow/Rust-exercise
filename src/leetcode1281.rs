use std::convert::TryFrom;
#[allow(dead_code)]
pub fn solution(n: i32) -> i32 {
    println!("Substract the Product and Sum of Digits of an Integer");

    let mut digit_product: i32 = 1;
    let mut digit_sum: i32 = 0;
    let mut m = n;
    let nsize: u32 = u32::try_from(n.to_string().len()).expect("number length is within u32 range");
    let base: i32 = 10;
    let mut divisor: i32 = base.pow(nsize - 1);

    while divisor > 0 {
        let now: i32 = m / divisor;
        m %= divisor;
        divisor /= base;
        digit_product *= now;
        digit_sum += now;
    }
    digit_product - digit_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1281_case1() {
        let n: i32 = 4421;
        let result = solution(n);
        let desired: i32 = 21;
        assert_eq!(result, desired);
    }
}
