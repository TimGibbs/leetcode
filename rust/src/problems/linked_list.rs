#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    #[inline]
    pub fn new_list(values: &[i32]) -> Option<Box<Self>> {
        if values.is_empty() { return None; }
        let next = if values.len() > 1 {
            ListNode::new_list(&values[1..])
        } else {
            None
        };
        Some(Box::new(ListNode {
            next,
            val: values[0]
        }))
    }
}