#[allow(dead_code)]
fn solution(address: &str) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1108_case1() {
        let address = String::from("1.1.1.1");
        let desired = String::from("1[.]1[.]1[.]1");

        assert_eq!(solution(&address), desired);
    }
}
