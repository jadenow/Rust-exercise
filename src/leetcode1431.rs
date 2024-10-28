fn solution(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    println!("Kids With the Greatest Number of Candies");

    let max_candies = candies.iter().max().unwrap();
    let result: Vec<bool> = candies
        .iter()
        .map(|&child| child + extra_candies >= *max_candies)
        .collect();

    result
}

#[test]
fn leetcode1431_case1() {
    let desired: Vec<bool> = vec![true, true, true, false, true];
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    assert_eq!(solution(candies, extra_candies), desired);
}
