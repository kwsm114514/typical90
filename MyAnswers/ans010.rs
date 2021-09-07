use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize}
    // class[i] 出席番号i番目までの点数の合計
    let mut class1: Vec<usize> = vec![0; n + 1];
    let mut class2: Vec<usize> = vec![0; n + 1];

    for i in 1..=n {
        input!{c: usize, p: usize}
        class1[i] += class1[i - 1];
        class2[i] += class2[i - 1];
        if c == 1 {
            class1[i] += p;
        }
        if c == 2 {
            class2[i] += p;
        }
    }

    input!{q: usize, range: [(usize, usize); q]}
    for i in 0..q {
        let ans1: usize = class1[range[i].1] - class1[range[i].0 - 1];
        let ans2: usize = class2[range[i].1] - class2[range[i].0 - 1];

        println!("{} {}", ans1, ans2);
    }
}
