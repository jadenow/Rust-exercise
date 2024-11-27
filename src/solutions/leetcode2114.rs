/// Finds the maximum number of words in a sentence from the input slice.
///
/// # Arguments
///
/// * `sentences` - A slice of `String` where each string represents a sentence.
///
/// # Errors
///
/// This function returns an error if the input slice is empty.
#[must_use = "The result must be used to ensure the minimum sum is calculated correctly."]
pub fn solution(sentences: &[String]) -> Result<usize, String> {
    sentences
        .iter()
        .map(|s| s.split_whitespace().count())
        .max()
        .ok_or_else(|| "No sentences provided".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode2114_case1() {
        let sentences: Vec<String> = vec![
            String::from("alice and bob love leetcode"),
            String::from("i think so too"),
            String::from("this is great thanks very much"),
        ];
        let desired = Ok(6);

        assert_eq!(solution(&sentences), desired);
    }
}
