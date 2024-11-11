use std::collections::BTreeMap;
#[allow(dead_code)]
fn solution(s: &str) -> String {
    let mut word_map = BTreeMap::new();

    s.split_whitespace().for_each(|word| {
        let letter = &word[..word.len() - 1];
        let num = word
            .chars()
            .last()
            .expect("number is smaller than 10")
            .to_digit(10)
            .expect("number is decimal");

        word_map.insert(num, letter.to_string());
    });

    word_map.values().cloned().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1859_case1() {
        let sentence: String = String::from("is2 sentence4 This1 a3");
        let desired: String = String::from("This is a sentence");
        assert_eq!(solution(&sentence), desired);
    }
}
