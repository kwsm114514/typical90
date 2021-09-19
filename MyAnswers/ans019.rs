use proconio::input;
use std::cmp::min;

// 区間dp
fn main() {
    input! {n: usize, a: [i64; 2 * n]}

    // dp[l][r] lからrまでを消すのに必要な最小のコスト
    let mut dp = vec![vec![1i64 << 60; 2 * n]; 2 * n];
    // 初期化
    for i in 0..n * 2 - 1 {
        dp[i][i + 1] = (a[i] - a[i + 1]).abs();
    }

    // 漸化式をもとにdp
    for i in 2..=n {
        let s = i * 2;
        for head in 0..=2 * n - s {
            let tail = head + s - 1;
            // headとtailの媒介：mid
            for mid in head + 2..=tail - 1 {
                dp[head][tail] = min(dp[head][tail], dp[head][mid - 1] + dp[mid][tail]);
            }
            let cost = (a[head] - a[tail]).abs();
            dp[head][tail] = min(dp[head][tail], dp[head + 1][tail - 1] + cost);

        }
    }

    println!("{}", dp[0][2 * n - 1]);
}
