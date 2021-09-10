use proconio::{fastout, input};
use petgraph::unionfind::UnionFind;

// グラフの連結問題にはUnionFind
#[fastout]
fn main() {
    input! {h: usize, w: usize, q: usize}

    let mut uf = UnionFind::new(h * w);
    let mut d = vec![false; h * w];

    let calc_index = |i, j| (i - 1) * w + (j - 1);

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {r: usize, c: usize}

            let index = calc_index(r, c);
            d[index] = true;
            let padding: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

            for &(i, j) in padding.iter() {
                let x = (r as i32 + i) as usize;
                let y = (c as i32 + j) as usize;
                if 0 < x && x <= h && 0 < y && y <= w {
                    let padding_index = calc_index(x, y);
                    if d[padding_index] {
                        uf.union(index, padding_index);
                    }
                }
            }
        } 

        if t == 2 {
            input! {ra: usize, rb: usize, ca: usize, cb: usize}
            let r = calc_index(ra, rb);
            let c = calc_index(ca, cb);
            println!("{}",
                if d[r] && d[c] && uf.equiv(r, c) {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}
