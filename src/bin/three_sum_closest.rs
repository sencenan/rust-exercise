// https://leetcode.com/problems/3sum-closest/

pub fn three_sum_closest(mut xs: Vec<i32>, target: i32) -> i32 {
  xs.sort();
  let mut res = xs[0] + xs[1] + xs[xs.len() - 1];

  for lo in 0..xs.len() - 2 {
    let mut mid = lo + 1;
    let mut hi = xs.len() - 1;

    while mid < hi {
      let cur = xs[lo] + xs[mid] + xs[hi];

      if cur > target {
        hi -= 1;
      } else {
        mid += 1;
      }

      if (cur - target).abs() < (res - target).abs() {
        res = cur;
      }
    }
  }

  res
}

fn main() {
  println!("{}", three_sum_closest(vec![-1, 2, 1, -4], 1));
  println!("{}", three_sum_closest(vec![0, 0, 0], 1));

  println!(
    "{}",
    three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2)
  );

  println!(
    "{}",
    three_sum_closest(
      vec![
        -738, -628, -94, -533, 605, 514, -345, 538, 719, -799, -46, 428, 286, 947, -842, 24, 870,
        -326, -712, 833, -21, 395, 968, 655, -112, -941, -741, 126, 908, 877, -5, 534, -834, -627,
        851, -632, 724, 831, -428, 140, 718, -703, -578, -403, -858, 871, 117, -494, -209, 812,
        885, 989, -17, 232, -888, -482, -943, 89, -964, 798, 321, -653, 828, -560, -318, -208, 81,
        4, 102, 685, 109, -126, -295, 888, -692, -210, 174, -95, 610, -713, 955, 368, -140, -57,
        -143, 555, 986, 455, -701, 513, -442, 879, 11, 987, 310, -687, -949, -558, -883, 504, 7,
        -356, -189, -667, -690, 892, -523, -379, -610, -748, 963, 667, -65, -325, 525, 616, 680,
        -983, 726, -172, 280, 364, 777, 789, -419, -488, -815, 518, 995, -199, -69, -440, 822,
        -477, 319, -40, 344, 739, -547, -362, -855, 654, -626, 849, -148, -60, 234, -945, -212,
        495, 129, -129, 835, -266, -464, 909, -726, -53, 668, 607, 897, -534, -213, 635, 471, -530,
        -826, -260, -673, -656, 523, -773, -438, 492, -469, -753, 355, -932, -630, 529, -414, -88,
        512, 553, -19, 556, -629, 872, 700, -201, 948, 369, -955, -747, -114, 925, -139, 692, 754,
        883, 600, -808, 847, -205, -918, -393, -735, 159, -430, -903, -727
      ],
      5969
    )
  );
}
