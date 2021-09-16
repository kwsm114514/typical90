use proconio::{input, fastout};
use crate::fenwick::FenwickTree;

//noinspection DuplicatedCode
mod fenwick {
    #[derive(Debug)]
    pub struct FenwickTree {
        tree: Vec<i64>,
    }

    impl FenwickTree {
        // O(n)
        pub fn new(n: usize) -> Self {
            Self {tree: vec![0; n + 1]}
        }

        // unstable
        pub fn from_slice(a: &[i64]) -> Self {
            let mut tree = vec![0; a.len()];
            for i in 1..=a.len() {
                let ai = a[i - 1];
                tree[i - 1] = tree[i - 1] + ai;
                let j = i + (i & i.wrapping_neg());
                if j <= a.len() {
                    tree[j - 1] = tree[i - 1] + tree[j - 1];
                }
            }
            Self{tree}
        }

        // iにxを足す。
        // O(log(n))
        pub fn add(&mut self, mut i: usize, x: i64) {
            i += 1;
            while i < self.tree.len() {
                self.tree[i] += x;
                i += i & i.wrapping_neg();
            }
        }

        // O(log(n))
        pub fn sum(&self, mut i: usize) -> i64 {
            i += 1;
            let mut result = 0;
            while i > 0 {
                result += self.tree[i];
                i ^= i & i.wrapping_neg();
            }
            result
        }

        // a1 + a2 + … + ai >= x となる最小のiをO(log(n))で求められる。
        // ただし、数列aの全要素が0以上
        // wが配列全体の合計を超えるとき配列を超えるindexを返す。
        // unstable
        pub fn lower_bound(&self, w: &i64) -> usize {
            if *w <= self.sum(0) {
                return 0;
            }
            let mut d = ((self.tree.len() + 1).next_power_of_two()) / 2;
            let mut x = 0;
            let mut acc = 0;
            while d != 0 {
                if d + x <= self.tree.len() && acc + self.tree[d + x - 1] < *w {
                    acc += self.tree[x + d - 1];
                    x += d;
                }
                d /= 2;
            }
            if x < self.tree.len() {
                x + 1
            } else {
                self.tree.len()
            }
        }
    }
}


#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut lrs: [(usize, usize); m],
    }

    lrs.sort_by(|a, b| {
        if a.0 < b.0 {
            std::cmp::Ordering::Less
        } else if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let mut fwt = FenwickTree::new(n + 1);

    let mut ans = 0;
    for &(l, r) in lrs.iter() {
        ans += fwt.sum(r - 1) - fwt.sum(l);
        fwt.add(r, 1);
    }

    println!("{}", ans);
}
