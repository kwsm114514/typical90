use proconio::input;
use petgraph::graph::Graph;
use petgraph::algo::tarjan_scc;

// 強連結成分分解(scc)をして互いに行き来できる頂点でグループ分けする。
// 辺の向きを逆にしても大丈夫という発想
fn main() {
    input!{_n: usize,m: usize, abs: [(u32, u32); m]}
    let graph = Graph::<u32, u32>::from_edges(&abs);

    let scc = tarjan_scc(&graph);
    let ans: usize = scc.iter().map(|i| i.len()*(i.len()-1)/2).sum();
    
    println!("{}", ans);
}
