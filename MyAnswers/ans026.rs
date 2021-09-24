use proconio::{input, fastout};

// dfsで色分けする。
// 二部グラフ
#[fastout]
fn main() {
    input!{n: usize}
    let mut graph: Vec<Vec<_>> = vec![vec![]; n];
    for _ in 1..n {
        input!{a: usize, b: usize}
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a -  1);
    }
    // -1: unvisited 0: 0から偶数の距離 1: 0から奇数の距離
    let mut colors: Vec<i64> = vec![-1; n];
    dfs(0usize, 0i64, &graph, &mut colors);

    let mut evens: Vec<_> = Vec::new();
    let mut odds: Vec<_> = Vec::new();
    for i in 0..n {
        if colors[i] == 0 {
            evens.push(i);
        }
        if colors[i] == 1 {
            odds.push(i);
        }
    }

    if evens.len() >= n / 2 {
        for i in 0..(n / 2) {
            print!("{} ", evens[i] + 1);
        }
    } else {
        for i in 0..(n / 2) {
            print!("{} ", odds[i] + 1);
        }
    }
}

fn dfs(node: usize, color: i64, graph: &Vec<Vec<usize>>, colors: &mut Vec<i64>) {
    colors[node] = color;
    // 隣接点を調べる。
    for &next in graph[node].iter() {
        // 訪れたことがあるなら
        if colors[next] != -1 {
            continue;
        }
        dfs(next, (color + 1) % 2, graph, colors);
    }
}
