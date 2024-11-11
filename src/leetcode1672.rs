#[allow(dead_code)]
fn solution(accounts: &[Vec<i32>]) -> i32 {
    println!("1672. Richest Customer Wealth");

    let ans = accounts
        .iter()
        .map(|person| person.iter().sum::<i32>())
        .max()
        .expect("Accounts vector is not empty");

    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1672_case1() {
        let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let desired = 6;
        assert_eq!(solution(&accounts), desired);
    }
}
