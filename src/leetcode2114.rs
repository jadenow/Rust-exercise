fn solution(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|s| s.split_whitespace().count())
        .max()
        .expect("Sentence is not empty") as i32
}

mod tests {
    use super::*;

    #[test]
    fn leetcode2114_case1() {
        let sentences: Vec<String> = vec![
            String::from("alice and bob love leetcode"),
            String::from("i think so too"),
            String::from("this is great thanks very much"),
        ];
        let desired: i32 = 6;

        assert_eq!(solution(sentences), desired);
    }
}
