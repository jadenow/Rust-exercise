#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[must_use]
pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    std::iter::successors(head, |node| node.next.clone())
        .map(|node| node.val)
        .fold(0, |acc, val| (acc << 1) | val)
}

#[must_use]
fn create_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    nums.iter()
        .rev()
        .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1290_case1() {
        let nums = vec![1, 0, 1];
        let head = create_linked_list(&nums);
        let desired = 5;

        assert_eq!(get_decimal_value(head), desired);
    }
}
