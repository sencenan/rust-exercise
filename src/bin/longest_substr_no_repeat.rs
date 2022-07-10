// https://leetcode.com/problems/longest-substring-without-repeating-characters/submissions/

use std::cmp;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
  let bytes = s.as_bytes();
  let mut start = 0;
  let mut max = 0;
  let mut last_seen = HashMap::new();

  for (i, &item) in bytes.iter().enumerate() {
    start = match last_seen.get(&item) {
      None => start,
      Some(idx) => cmp::max(start, idx + 1),
    };

    max = cmp::max(max, i - start + 1);

    last_seen.insert(item, i);
  }

  max as i32
}

fn main() {
  // println!("{}", length_of_longest_substring("abcbef".to_string()));
  // println!("{}", length_of_longest_substring("pwwkew".to_string()));
  // println!("{}", length_of_longest_substring("bbbbbb".to_string()));
  // println!("{}", length_of_longest_substring("abcabcbb".to_string()));
  // println!("{}", length_of_longest_substring("".to_string()));
  println!("{}", length_of_longest_substring("abba".to_string()));
}
