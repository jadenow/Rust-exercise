/// Computes the difference between the product and the sum of the digits of an integer.
///
/// # Arguments
/// * `n` - A non-negative integer whose digits will be processed.
///
/// # Returns
/// * `Ok<u32>` - If the computation is successful, the result is returned as a `u32`.
/// * `Err<&'static str>` - If an invalid digit is encountered during processing.
///
/// # Errors
/// This function returns an error if the conversion of characters to digits fails. For example,
/// if the input contains invalid characters (not a concern for `u32` input as all characters are digits).
///
#[must_use = "The result of this function should be used to ensure correct program behavior."]
pub fn solution(n: u32) -> Result<u32, &'static str> {
    println!("Subtract the Product and Sum of Digits of an Integer");

    let (digit_product, digit_sum) = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).ok_or("Invalid digit")) // Directly get Option<u32> and handle errors
        .collect::<Result<Vec<_>, _>>()? // Collect valid results or propagate errors
        .into_iter()
        .fold((1, 0), |(prod, sum), digit| (prod * digit, sum + digit));

    Ok(digit_product - digit_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1281_case1() {
        let n: u32 = 4421;
        let desired: Result<u32, &'static str> = Ok(21); // Define desired as `Ok<u32>`

        assert_eq!(solution(n), desired); // Compare result directly with desired
    }
}
