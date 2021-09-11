use crate::dijkstra::{dijkstra, Edge};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    };

    let mut graph = vec![vec![]; n];
    for (a, b, c) in &abc {
        graph[a - 1].push(Edge::new(*b - 1, *c));
        graph[b - 1].push(Edge::new(*a - 1, *c));
    }

    let dist_from_0 = dijkstra(&graph, 0);
    let dist_from_n = dijkstra(&graph, n - 1);

    for i in 0..n {
        let ans = dist_from_0[i] + dist_from_n[i];
        println!("{}", ans);
    }
}
mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[derive(Debug, Clone, Copy)]
    pub struct Edge {
        to: usize,
        weight: usize,
    }

    impl Edge {
        pub fn new(to: usize, weight: usize) -> Edge {
            Edge { to, weight }
        }
    }

    pub fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
        const INF: usize = 1_000_000_000 + 7;
        let size = graph.len();
        let mut queue = BinaryHeap::new();
        let mut dist = vec![INF; size];

        queue.push(Reverse((0, start)));
        dist[start] = 0;

        while !queue.is_empty() {
            let Reverse((d, v)) = queue.pop().unwrap();
            if dist[v] < d {
                continue;
            }
            for Edge { to, weight } in &graph[v] {
                if dist[*to] > dist[v] + *weight {
                    dist[*to] = dist[v] + *weight;
                    queue.push(Reverse((dist[*to], *to)));
                }
            }
        }

        dist
    }
}
