use proconio::{fastout, input};

const length: usize = 1000;

#[fastout]
fn main() {
    input! {n: usize, paper: [(usize, usize, usize, usize); n]}
    let mut tiles: Vec<Vec<usize>> = vec![vec![0; length + 1]; length + 1];
    // 二次元いもす法とかいう変なアルゴリズム
    for &(lx, ly, rx, ry) in paper.iter() {
        tiles[ly][lx] += 1;
        tiles[ly][rx] -= 1;
        tiles[ry][lx] -= 1;
        tiles[ry][rx] += 1;
    }
    // 二次元いもす: 軸ごとに足す。
    for y in 0..=length {
        for x in 1..=length {
            tiles[y][x] += tiles[y][x - 1];
        }
    }
    for x in 0..=length {
        for y in 1..=length {
            tiles[y][x] += tiles[y - 1][x];
        }
    }

    let mut counts: Vec<usize> = vec![0; n + 1];
    for i in 0..=length {
        for &tile in tiles[i].iter() {
            counts[tile] += 1;
        }
    }
    for i in 1..=n {
        println!("{}", counts[i]);
    }
}
