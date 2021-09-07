use proconio::input;

fn main() {
    input!{n: usize}
    let mut tonari: Vec<Vec<_>> = vec![vec![]; n];
    for _ in 1..n {
        input!{a: usize, b: usize}
        tonari[a - 1].push(b - 1);
        tonari[b - 1].push(a - 1);
    }
    // (index, distance)
    let deepest: (usize, usize) = bfs(&tonari, 0, n, n);
    let furthest: (usize, usize) = bfs(&tonari, deepest.0, n, n);
    
    println!("{}", furthest.1 + 1);
}

fn bfs(tonari: &Vec<Vec<usize>>, start: usize, parent: usize, n: usize) -> (usize, usize) {
    let mut dist: Vec<usize> = vec![0; n];
    let mut que = std::collections::VecDeque::new();
    que.push_back((start, n));

    while let Some((u, p)) = que.pop_front() {
        for &next in tonari[u].iter() {
            if next == p {
                continue;
            }
            if dist[next] < dist[u] + 1 {
                dist[next] = dist[u] + 1;
                que.push_back((next, u));
            }
        }
    }

    let mut max_index: usize = n;
    let mut max_depth: usize = 0;
    for i in 0..n {
        if max_depth < dist[i] {
            max_index = i;
            max_depth = dist[i];
        }
    }

    return (max_index, max_depth);
}
