use std::collections::HashMap;
fn solution(nums: Vec<i32>) -> Vec<i32> {
    println!("1365. How Many Numbers Are Smaller Than the Current Number");

    let mut indices: Vec<usize> = (0..nums.len()).collect();
    indices.sort_by_key(|&i| nums[i]);

    let mut counter = HashMap::new();

    for (rank, &i) in indices.iter().enumerate() {
        counter.entry(nums[i]).or_insert(rank as i32);
    }

    nums.iter()
        .map(|&n| {
            *counter
                .get(&n)
                .expect("Expected value to be present in hashmap do to sorting and counting")
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn leetcode1365_case1() {
        let nums: Vec<i32> = vec![6, 5, 4, 8];
        let desired: Vec<i32> = vec![2, 1, 0, 3];
        assert_eq!(solution(nums), desired);
    }
}
