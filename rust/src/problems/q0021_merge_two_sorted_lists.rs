use crate::problems::linked_list::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, Some(l2)) => Some(l2),
        (Some(l1), None) => Some(l1),
        (Some(mut l1), Some(mut l2)) => {
            if l1.val < l2.val {
                l1.next = merge_two_lists(l1.next, Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists(Some(l1), l2.next);
                Some(l2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let a = ListNode::new_list(&[1,2,4]);
        let b = ListNode::new_list(&[1,3,4]);
        let e = ListNode::new_list(&[1,1,2,3,4,4]);

        assert_eq!(merge_two_lists(a,b), e);
    }
    #[test]
    fn case2() {
        let a = ListNode::new_list(&[]);
        let b = ListNode::new_list(&[]);
        let e = ListNode::new_list(&[]);
        assert_eq!(merge_two_lists(a,b), e);

    }
    #[test]
    fn case3() {
        let a = ListNode::new_list(&[]);
        let b = ListNode::new_list(&[0]);
        let e = ListNode::new_list(&[0]);

        assert_eq!(merge_two_lists(a,b), e);
    }
}