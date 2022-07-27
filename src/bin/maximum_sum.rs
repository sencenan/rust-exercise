// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/

use std::cmp;
use std::collections::HashMap;

pub fn maximum_sum(xs: Vec<i32>) -> i32 {
  fn sum_of_digits(num: i32) -> i32 {
    if num < 10 {
      num
    } else {
      num % 10 + sum_of_digits(num / 10)
    }
  }

  let mut max = -1;
  let mut max_of_sum = HashMap::new();

  for x in xs {
    let s = sum_of_digits(x);

    match max_of_sum.get(&s) {
      None => {
        max_of_sum.insert(s, x);
      }
      Some(prev_max) => {
        max = cmp::max(max, prev_max + x);
        max_of_sum.insert(s, cmp::max(*prev_max, x));
      }
    }
  }

  max
}

fn main() {
  println!("{}", maximum_sum(vec![18, 43, 36, 13, 7]));
  println!("{}", maximum_sum(vec![10, 12, 19, 14]));
}
