/// Calculates the maximum wealth among all customers in the accounts.
///
/// # Errors
///
/// This function returns an error if:
/// - The accounts slice is empty.
/// - The computation of the maximum wealth fails.
#[must_use = "The result must be used to ensure the calculated maximum wealth is not ignored."]
pub fn solution(accounts: &[Vec<i32>]) -> Result<i32, String> {
    println!("1672. Richest Customer Wealth");

    if accounts.is_empty() {
        return Err("No customers in the accounts".to_string());
    }
    let ans = accounts
        .iter()
        .map(|person| person.iter().sum::<i32>())
        .max()
        .ok_or_else(|| "Failed to compute maximum wealth".to_string())?;

    Ok(ans)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1672_case1() {
        let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let desired = Ok(6);
        assert_eq!(solution(&accounts), desired);
    }
}
