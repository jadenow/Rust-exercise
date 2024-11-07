use std::collections::HashMap;
fn solution(key: String, message: String) -> String {
    let mut table = HashMap::new();
    key.chars()
        .filter(|&c| c != ' ')
        .scan('a', |now, c| {
            let entry = table.entry(c).or_insert_with(|| {
                let tmp = *now;
                *now = ((*now as u8) + 1) as char;
                tmp
            });
            Some(*entry)
        })
        .for_each(drop);

    message
        .chars()
        .map(|c| {
            if let Some(&val) = table.get(&c) {
                val
            } else {
                c
            }
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn leetcode2325_case1() {
        let key: String = String::from("the quick brown fox jumps over the lazy dog");
        let message: String = String::from("vkbs bs t suepuv");
        let desired = String::from("this is a secret");

        assert_eq!(solution(key, message), desired);
    }
}
