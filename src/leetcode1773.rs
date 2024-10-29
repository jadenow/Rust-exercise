use std::collections::HashMap;
fn solution(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut rules = HashMap::new();
    rules.insert(String::from("type"), 0 as usize);
    rules.insert(String::from("color"), 1 as usize);
    rules.insert(String::from("name"), 2 as usize);

    items
        .iter()
        .filter(|item| {
            item[*rules
                .get(&rule_key)
                .expect("rule key is type, color, or name")]
                == rule_value
        })
        .count() as i32
}

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
        assert_eq!(solution(items, rule_key, rule_value), desired);
    }
}
