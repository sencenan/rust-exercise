// https://leetcode.com/problems/zigzag-conversion/

pub fn convert(s: String, num_rows: i32) -> String {
  if num_rows == 1 {
    return s;
  };

  let num_rows = num_rows as usize; // shadowing
  let mut rows = Vec::with_capacity(num_rows);
  for _ in 0..num_rows {
    rows.push(String::new());
  }

  let mut row_idx = 0;

  for c in s.chars() {
    if row_idx < num_rows {
      // down stroke
      rows[row_idx].push(c);
    } else {
      // up stroke
      rows[num_rows * 2 - row_idx - 2].push(c);
    }

    row_idx = (row_idx + 1) % (num_rows * 2 - 2);
  }

  rows.join("")
}

fn main() {
  println!("{}", convert("abcdefghijk".to_string(), 4));
  println!("{}", convert("ab".to_string(), 1));
  println!("{}", convert("ab".to_string(), 2));
  println!("{}", convert("a".to_string(), 1));
}
