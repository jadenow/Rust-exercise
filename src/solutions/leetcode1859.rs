use std::collections::BTreeMap;
#[must_use]
pub fn solution(s: &str) -> String {
    s.split_whitespace()
        .filter_map(|word| {
            let (letter, num) = word.split_at(word.len() - 1);
            num.parse::<usize>().ok().map(|num| (num, letter))
        })
        .collect::<BTreeMap<_, _>>()
        .values()
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
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
