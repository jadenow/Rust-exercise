#[must_use]
pub fn solution(n: i32) -> i32 {
    std::iter::successors(Some(n), |&current| {
        if current > 1 {
            Some(if current % 2 == 0 {
                current / 2
            } else {
                (current - 1) / 2 + 1
            })
        } else {
            None
        }
    })
    .take_while(|&current| current > 1)
    .fold(0, |cnt, current| {
        cnt + if current % 2 == 0 {
            current / 2
        } else {
            (current - 1) / 2
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1688_case1() {
        let n: i32 = 14;
        let desired: i32 = 13;

        assert_eq!(solution(n), desired);
    }
}
