#[must_use]
pub fn solution(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    println!("Kids With the Greatest Number of Candies");
    if let Some(&max_candies) = candies.iter().max() {
        candies
            .iter()
            .map(|&child| child + extra_candies >= max_candies)
            .collect()
    } else {
        Vec::new() // Return an empty vector if the slice is empty
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1431_case1() {
        let desired: Vec<bool> = vec![true, true, true, false, true];
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        assert_eq!(solution(&candies, extra_candies), desired);
    }
}
