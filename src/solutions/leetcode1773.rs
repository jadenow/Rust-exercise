/// Finds the number of items that match the given rule key and value.
///
/// # Arguments
///
/// * `items` - A slice of vectors, where each vector represents an item with its type, color, and name.
/// * `rule_key` - The key to filter items by ("type", "color", or "name").
/// * `rule_value` - The value to match against the specified rule key.
///
/// # Returns
///
/// A `Result<usize, String>` where:
/// - `Ok(usize)` contains the count of matching items.
/// - `Err(String)` contains an error message if the rule key is invalid.
///
/// # Errors
///
/// Returns an error if the `rule_key` is not "type", "color", or "name".
///
#[must_use = "The return value indicates success or failure. Please handle it explicitly."]
pub fn solution(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> Result<usize, String> {
    let rule_index = if rule_key == "type" {
        Some(0)
    } else if rule_key == "color" {
        Some(1)
    } else if rule_key == "name" {
        Some(2)
    } else {
        None
    };

    rule_index
        .ok_or_else(|| format!("Invalid rule_key: {rule_key}"))
        .map(|index| {
            items.iter().fold(0, |acc, item| {
                if let Some(value) = item.get(index) {
                    if value == rule_value {
                        return acc + 1;
                    }
                }
                acc
            })
        })
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
        let desired = Ok(1);

        assert_eq!(solution(&items, &rule_key, &rule_value), desired);
    }
}
