use crate::problems::linked_list::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.as_mut();

    while let Some(node) = current {
        while let Some(next_node) = node.next.as_mut() {
            if next_node.val == node.val {
                node.next = next_node.next.take(); // Remove the duplicate node
            } else {
                break; // Move to the next distinct node
            }
        }
        current = node.next.as_mut();
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let i = Some(Box::new(ListNode::new_list(&[1,1,2])));
        let a = Some(Box::new(ListNode::new_list(&[1,2])));
        assert_eq!(delete_duplicates(i),a)
    }
    #[test]
    fn case2() {
        let i = Some(Box::new(ListNode::new_list(&[1,1,2,3,3])));
        let a = Some(Box::new(ListNode::new_list(&[1,2,3])));
        assert_eq!(delete_duplicates(i),a);
    }

}