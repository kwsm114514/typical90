use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u32; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }
    // タスキを渡せるか？
    let mut cant = vec![vec![false; n]; n];
    for i in 0..m {
        let (x, y) = xy[i];
        cant[x - 1][y - 1] = true;
        cant[y - 1][x - 1] = true;
    }

    let mut res = std::u32::MAX;
    for p in (0..n).permutations(n) {
        let mut sum = a[p[0]][0];
        let mut ok = true;
        for i in 1..n {
            let prev = p[i - 1];
            let curr = p[i];
            sum += a[curr][i];
            if cant[curr][prev] || cant[prev][curr] {
                ok = false;
            }
        }
        if ok && res > sum {
            res = sum;
        }
    }

    if res == std::u32::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
