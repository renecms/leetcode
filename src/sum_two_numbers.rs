// Definition for singly-linked list.
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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut current_l1 = l1;
    let mut current_l2 = l2;
    let mut result = None;
    let mut carry_one = 0;
    
    while current_l1.is_some() || current_l2.is_some() {
        let sum = match (&current_l1, &current_l2) {
            (None, None) => carry_one,
            (None, Some(l2)) => l2.val + carry_one,
            (Some(l1), None) => l1.val + carry_one,
            (Some(l1), Some(l2)) => l1.val + l2.val + carry_one,
        };
        carry_one = sum / 10;
        let sum = sum % 10;
        add_val_to_list(&mut result, sum);
        current_l1 = if current_l1.is_none() { None } else { current_l1.unwrap().next };
        current_l2 = if current_l2.is_none() { None } else { current_l2.unwrap().next };
    }
    if carry_one == 1 {
        add_val_to_list(&mut result, 1);
    }
    return result;
}

fn add_val_to_list(list: &mut Option<Box<ListNode>>, val: i32) {
    if list.is_none()  {
        *list = Some(Box::new(ListNode::new(val)));
        return;
    }

    let mut next_node = &mut list.as_mut().unwrap().next;

    match &mut next_node {
        Some(_) => add_val_to_list(next_node, val),
        None => *next_node = Some(Box::new(ListNode::new(val))) ,
    }
}

#[test]
fn test() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let result = ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 8, next: None })),
        })),
    };
    assert_eq!(result, *add_two_numbers(l1, l2).unwrap());
}