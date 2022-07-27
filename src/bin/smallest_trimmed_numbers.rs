// https://leetcode.com/problems/query-kth-smallest-trimmed-number/

use std::cmp::Ordering;
use std::collections::HashMap;

pub fn smallest_trimmed_numbers(ss: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
  let mut sorted = HashMap::new();
  let num_len = ss[0].len();
  let xs: Vec<_> = ss.iter().map(|x| x.clone().into_bytes()).collect();

  let trim_and_sort = |trim: i32| {
    let mut res: Vec<_> = xs
      .iter()
      .enumerate()
      .map(|(idx, it)| (idx as i32, it))
      .collect();

    res.sort_by(|(_, a), (_, b)| {
      let mut i = num_len - (trim as usize);

      while i < num_len {
        let ord = (*a)[i].cmp(&(*b)[i]);
        match ord {
          Ordering::Equal => i += 1,
          _ => return ord,
        }
      }

      Ordering::Equal
    });

    res
  };

  queries
    .iter()
    .map(|query| {
      let k = query[0] as usize;
      let trim = query[1];

      let (idx, _) = sorted.entry(trim).or_insert_with(|| trim_and_sort(trim))[k - 1];
      idx
    })
    .collect()
}

fn main() {
  // println!(
  //   "{:?}",
  //   smallest_trimmed_numbers(
  //     vec!["102", "473", "251", "814"]
  //       .iter()
  //       .map(|x| x.to_string())
  //       .collect(),
  //     vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]]
  //   )
  // );

  // println!(
  //   "{:?}",
  //   smallest_trimmed_numbers(
  //     vec!["24", "37", "96", "04"]
  //       .iter()
  //       .map(|x| x.to_string())
  //       .collect(),
  //     vec![vec![2, 1], vec![2, 2]]
  //   )
  // );

  println!(
    "{:?}",
    smallest_trimmed_numbers(
      vec![
        "89288488870527604910029",
        "36097185739782752175822",
        "66626740310751086142991",
        "39210310199276100943112",
        "27763774389382535382104",
        "38417381130871656561097",
        "88045540666254006395713",
        "95788007927435793172832",
        "15831923319620654311625",
        "45043895456202186804606",
        "87291364237858759125697",
        "88163116582250002569968",
        "00507332488046565482379",
        "57170661333341274356658",
        "06401271520738451116188",
        "21731485952024837866860"
      ]
      .iter()
      .map(|x| x.to_string())
      .collect(),
      vec![
        vec![3, 17],
        vec![10, 22],
        vec![13, 21],
        vec![12, 16],
        vec![1, 23],
        vec![10, 19],
        vec![12, 21],
        vec![10, 5],
        vec![12, 9],
        vec![12, 10],
        vec![9, 5],
        vec![12, 8],
        vec![14, 6],
        vec![5, 10],
        vec![11, 4],
        vec![15, 15],
        vec![13, 9],
        vec![1, 19],
        vec![5, 12],
        vec![10, 15],
        vec![4, 18],
        vec![4, 3],
        vec![16, 13],
        vec![6, 19],
        vec![4, 18],
        vec![2, 6],
        vec![15, 12]
      ]
    )
  );
}
