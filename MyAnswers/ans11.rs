use proconio::input;

#[derive(Debug, Copy, Clone)]
struct Job {
    due: usize, span: usize, reward: usize
}

fn main() {
    input! {n: usize}
    let mut dcs: Vec<_> = Vec::new();
    for _ in 0..n {
        input!{d: usize, c: usize, s: usize}
        dcs.push(Job{due: d, span: c, reward: s});
    }

    dcs.sort_by(|a, b| a.due.cmp(&b.due));

    let mut dp = vec![0; 5001];

    for i in 0..n {
        if dcs[i].due < dcs[i].span {
            continue;
        }
        for j in (1..(dcs[i].due - dcs[i].span + 1).min(dp.len())).rev() {
            if dp[j] > 0 {
                dp[j + dcs[i].span] = dp[j + dcs[i].span].max(dp[j] + dcs[i].reward);
            }
        }
        dp[dcs[i].span] = dp[dcs[i].span].max(dcs[i].reward);
    }

    println!("{}", dp.iter().max().unwrap());
}
