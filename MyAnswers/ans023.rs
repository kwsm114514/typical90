use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {h: usize, w: usize,field: [Chars; h]}
    
    let mask: usize = (1 << (w + 1)) - 1;

    let forbidden_place: [usize; 3] = [
        if w == 1 { 1 } else { 3 << (w - 2) },
        (3 << (w - 1)) + 1,
        (7 << (w - 2)) + 1,
    ];

    let mut dp_from = &mut HashMap::<usize, usize>::new();
    let mut dp_to = &mut HashMap::<usize, usize>::new();
    dp_from.insert(0, 1);

    for i in 0..h {
        for j in 0..w {
            let placeable = field[i][j] == '.';
            let fp = forbidden_place[if j == 0 { 0 } else if j == w - 1 { 1 } else { 2 }];

            for (place, pattern) in dp_from.iter() {
                let new_place = (place << 1) & mask;
                *dp_to.entry(new_place).or_insert(0) += pattern;
                if placeable && place & fp == 0 {
                    *dp_to.entry(new_place | 1).or_insert(0) += pattern;
                }
            }

            for v in dp_to.values_mut() {
                *v = *v % MOD;
            }

            // 入れ替える
            std::mem::swap(&mut dp_from, &mut dp_to);
            dp_to.clear();
        }
    }

    let ans = dp_from.values().fold(0, |a, b| (a + *b) % MOD);

    println!("{}", ans);
}
