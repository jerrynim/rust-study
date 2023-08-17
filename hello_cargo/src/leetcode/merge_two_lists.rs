#[allow(dead_code)]
pub fn solution(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // match (list1, list2) {
    //     (None, None) => None,
    //     (Some(n), None) | (None, Some(n)) => Some(n),
    //     (Some(l1), Some(l2)) => {
    //         if l1.val >= l2.val {
    //             Some(Box::new(ListNode {
    //                 val: l2.val,
    //                 next: Solution::merge_two_lists(Some(l1), l2.next),
    //             }))
    //         } else {
    //             Some(Box::new(ListNode {
    //                 val: l1.val,
    //                 next: Solution::merge_two_lists(l1.next, Some(l2)),
    //             }))
    //         }
    //     }
    // }

    match (list1, list2) {
        (None, Node) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val >= l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: solution(l1.next, Some(l2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: solution(l1.next, Some(l2)),
                }))
            }
        }
    }
}
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
