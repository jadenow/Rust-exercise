#[must_use]
pub fn solution(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> usize {
    let rule_index = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return 0,
    };

    items
        .iter()
        .filter(|item| {
            if let Some(value) = item.get(rule_index) {
                value == rule_value
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1773_case1() {
        let items: Vec<Vec<String>> = vec![
            vec![
                String::from("phone"),
                String::from("blue"),
                String::from("pixel"),
            ],
            vec![
                String::from("computer"),
                String::from("silver"),
                String::from("lenovo"),
            ],
            vec![
                String::from("phone"),
                String::from("gold"),
                String::from("iphone"),
            ],
        ];
        let rule_key = String::from("color");
        let rule_value = String::from("silver");
        let desired = 1;
        assert_eq!(solution(&items, &rule_key, &rule_value), desired);
    }
}
