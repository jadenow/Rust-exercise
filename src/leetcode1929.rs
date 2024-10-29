fn solution(nums: Vec<i32>) -> Vec<i32> {
    println!("1929. Concatenation of Array");

    let mut ans: Vec<i32> = nums.clone();
    ans.extend(&nums);

    ans
}

#[test]
fn leetcode1929_case1() {
    let nums: Vec<i32> = vec![1, 2, 1];
    let desired = vec![1, 2, 1, 1, 2, 1];
    assert_eq!(solution(nums), desired);
}
