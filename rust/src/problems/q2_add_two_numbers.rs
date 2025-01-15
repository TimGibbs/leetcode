use crate::problems::linked_list::ListNode;

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut current = &mut dummy_head;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let l1_val = l1.as_ref().map_or(0, |node| node.val);
        let l2_val = l2.as_ref().map_or(0, |node| node.val);

        let sum = l1_val + l2_val + carry;
        carry = sum / 10;
        let new_val = sum % 10;

        current.next = Some(Box::new(ListNode::new(new_val)));
        current = current.next.as_mut().unwrap();

        l1 = l1.and_then(|node| node.next);
        l2 = l2.and_then(|node| node.next);
    }

    dummy_head.next
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let a = Some(Box::new(ListNode::new_list(&[2, 4, 3])));
        let b = Some(Box::new(ListNode::new_list(&[5, 6, 4])));
        let e = Some(Box::new(ListNode::new_list(&[7, 0, 8])));

        assert_eq!(add_two_numbers(a,b), e);
    }
    #[test]
    fn case2() {
        let a = Some(Box::new(ListNode::new(0)));
        let b = Some(Box::new(ListNode::new(0)));
        let e = Some(Box::new(ListNode::new(0)));

        assert_eq!(add_two_numbers(a,b), e);
    }
    #[test]
    fn case3() {
        let a = Some(Box::new(ListNode::new_list(&[9,9,9,9,9,9,9])));
        let b = Some(Box::new(ListNode::new_list(&[9,9,9,9])));
        let e = Some(Box::new(ListNode::new_list(&[8,9,9,9,0,0,0,1])));

        assert_eq!(add_two_numbers(a,b), e);
    }
}