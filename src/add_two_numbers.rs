#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let unwrap_or_zero = |node_ref: &Option<Box<ListNode>>| -> i32 {
            return match *node_ref {
                None => 0,
                Some(_) => node_ref.as_ref().unwrap().val
            };
        };
        let mut root: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut r_pointer: &mut Option<Box<ListNode>> = &mut root;
        let mut overflow = false;
        let mut pointer1: &Option<Box<ListNode>> = &l1;
        let mut pointer2: &Option<Box<ListNode>> = &l2;
        while *pointer1 != None || *pointer2 != None {
            let addition_result = add(
                unwrap_or_zero(pointer1),
                unwrap_or_zero(pointer2),
                &overflow,
            );
            overflow = addition_result.overflow;
            if *pointer1 != None {
                pointer1 = &pointer1.as_ref().unwrap().next;
            }
            if *pointer2 != None {
                pointer2 = &pointer2.as_ref().unwrap().next;
            }
            r_pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(addition_result.val)));
            r_pointer = &mut r_pointer.as_mut().unwrap().next;
        }
        if overflow {
            r_pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        }
        return root.unwrap().next;
    }
}

struct AdditionResult {
    pub val: i32,
    pub overflow: bool,
}

fn add(a: i32, b: i32, decimal_overflow: &bool) -> AdditionResult {
    if a >= 10 || b >= 10 {
        panic!("Supported values are 1..9")
    }
    let mut sum = a + b;
    if *decimal_overflow {
        sum += 1;
    }
    if sum > 9 {
        AdditionResult { val: sum % 10, overflow: true }
    } else {
        AdditionResult { val: sum, overflow: false }
    }
}

fn number_to_list_node(val: i32) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut r_pointer: &mut Option<Box<ListNode>> = &mut root;
    let digits: Vec<i32> = val.to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    for digit in digits {
        r_pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(digit)));
        r_pointer = &mut r_pointer.as_mut().unwrap().next;
    }
    root.unwrap().next
}

#[test]
fn number_to_list_node_test() {
    let mut node = Some(Box::new(ListNode::new(0)));
    node.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    assert_eq!(
        node,
        number_to_list_node(10)
    )
}

#[test]
fn add_two_numbers_test_1() {
    assert_eq!(
        Solution::add_two_numbers(
            number_to_list_node(99),
            number_to_list_node(1),
        ),
        number_to_list_node(100)
    )
}

#[test]
fn add_two_numbers_test_2() {
    assert_eq!(
        Solution::add_two_numbers(
            number_to_list_node(120),
            number_to_list_node(12),
        ),
        number_to_list_node(132)
    )
}

#[test]
fn add_two_numbers_test_3() {
    assert_eq!(
        Solution::add_two_numbers(
            number_to_list_node(10),
            None,
        ),
        number_to_list_node(10)
    )
}

#[test]
fn add_test_1() {
    let result = add(1, 2, &false);
    assert_eq!(result.val, 3);
    assert!(!result.overflow);
}

#[test]
fn add_test_2() {
    let result = add(5, 6, &false);
    assert_eq!(result.val, 1);
    assert!(result.overflow);
}

#[test]
fn add_test_3() {
    let result = add(5, 7, &true);
    assert_eq!(result.val, 3);
    assert!(result.overflow);
}