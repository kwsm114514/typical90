use std::collections::VecDeque;
use proconio::{fastout, input};

// bfsで色分けする。
// 二部グラフ
#[fastout]
fn main() {
    input! {n: usize, ab: [(usize, usize); n - 1]}

    let mut adj_list = vec![vec![]; n];
    let mut g = vec![0; n];

    for (a, b) in ab.iter() {
        adj_list[a - 1].push(b - 1);
        adj_list[b - 1].push(a - 1);
    }

    let mut stack = VecDeque::new();

    stack.push_back((0, 1));

    let mut v1 = vec![];
    let mut v2 = vec![];

    while let Some((index, color)) = stack.pop_back() {
        if g[index] != 0 {
            continue;
        }

        g[index] = color;

        if color == 1 {
            v1.push(index);
        } else {
            v2.push(index);
        }

        for &i in adj_list[index].iter() {
            let new_color = if color == 1 { 2 } else { 1 };
            stack.push_back((i, new_color));
        }
    }

    let target = if v1.len() > v2.len() { v1 } else { v2 };

    for i in 0..n / 2 {
        print!("{} ", target[i] + 1)
    }
}
