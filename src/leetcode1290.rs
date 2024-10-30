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

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut now = head;
    let mut result = 0;

    while let Some(node) = now {
        result = (result << 1) | node.val;
        now = node.next;
    }
    result
}

//create a linked list fot testing
fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut now = &mut head;

    for &val in nums.iter() {
        let new_node = Box::new(ListNode::new(val));
        *now = Some(new_node);
        now = &mut now.as_mut().expect("this node has a next link").next;
    }

    head
}

mod tests {
    use super::*;

    #[test]
    fn leetcode1290_case1() {
        let head = create_linked_list(vec![1, 0, 1]);
        let desired = 5;

        assert_eq!(get_decimal_value(head), desired);
    }
}
