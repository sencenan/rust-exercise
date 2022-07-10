// https://leetcode.com/problems/add-two-numbers/

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
  // add to form new linked list
  fn zip(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (a, b) {
      (None, None) => None,
      (None, Some(ls)) => Some(ls),
      (Some(ls), None) => Some(ls),
      (Some(a), Some(b)) => Some(Box::new(ListNode {
        val: a.val + b.val,
        next: zip(a.next, b.next),
      })),
    }
  }

  // run carry over until it's done
  fn carry(a: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match a {
      None => None,
      Some(node) => {
        let ns = match node.next {
          None => {
            if node.val < 10 {
              None
            } else {
              Some(Box::new(ListNode::new(node.val / 10)))
            }
          }
          Some(x) => Some(Box::new(ListNode {
            val: node.val / 10 + x.val,
            next: x.next,
          })),
        };

        Some(Box::new(ListNode {
          val: node.val % 10,
          next: carry(ns),
        }))
      }
    }
  }

  carry(zip(l1, l2))
}

fn to_list(xs: &[i32]) -> Option<Box<ListNode>> {
  if xs.is_empty() {
    None
  } else {
    Some(Box::new(ListNode {
      val: xs[0],
      next: to_list(&xs[1..]),
    }))
  }
}

fn main() {
  let a = to_list(&[9, 9, 9, 9, 9, 9, 9]);
  let b = to_list(&[9, 9, 9]);

  let c = add_two_numbers(a, b);

  match c {
    Some(x) => println!("{:?}", x),
    None => (),
  };
}
