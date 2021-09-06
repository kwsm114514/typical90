use proconio::{input, marker::Chars};
const MOD: usize = 1e9 as usize + 7;

fn main() {
    input!{n: usize, s: Chars}
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; 8];
    let atcoder: Vec<char> = vec!['a', 't', 'c', 'o', 'd', 'e', 'r',];
    // 初期値を設定する。
    for j in 0..=n {
        dp[0][j] = 1;
    }

    // dp
    for i in 1..=7 {
        for j in 1..=n {
            dp[i][j] += dp[i][j - 1];
            if atcoder[i - 1] == s[j - 1] {
                dp[i][j] += dp[i - 1][j - 1];
                dp[i][j] %= MOD;
            } 
        }
    }

    let ans: usize = dp[7][n];
    println!("{}", ans);
}
