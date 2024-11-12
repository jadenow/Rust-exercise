/// Calculates the minimum sum of a four-digit number after splitting its digits.
///
/// # Errors
///
/// Returns an error if the input string does not represent a valid four-digit number.
///
#[must_use = "The result must be used to ensure the minimum sum is calculated correctly."]
pub fn solution(num: &str) -> Result<u32, String> {
    println!("2160. Minimum Sum of Four Digit Number After Splitting Digits");

    let mut digits = num
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .ok_or("Input must be a valid digit".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    if digits.len() != 4 {
        return Err("Input must be a four-digit number".to_string());
    }

    digits.sort_unstable();

    let n1 = digits[0] * 10 + digits[2];
    let n2 = digits[1] * 10 + digits[3];

    Ok(n1 + n2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode2160_case1() {
        let num = "4009";
        let desired = Ok(13);
        assert_eq!(solution(&num), desired);
    }
}
