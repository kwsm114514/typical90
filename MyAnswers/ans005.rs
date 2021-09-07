use proconio::input;
const MOD: usize = 1e9 as usize + 7;

// 動的計画法で答えを出す。
fn main() {
    // n: 桁数、b: 倍数、k: 使える数字の個数、nums: 使える数字
    input!{n: usize, b: usize, k: usize, nums: [usize; k]}
    // dp[i][j] i: 上から何桁目か　j:bで割ったときのあまり
    let mut dp: Vec<Vec<usize>> = vec![vec![0; b]; n + 1];
    // 初期値を設定する。
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..b {
            for num in 0..k {
                let next_amari: usize = (10 * j + nums[num]) % b;
                dp[i + 1][next_amari] += dp[i][j];
                dp[i + 1][next_amari] %= MOD;
            }
        }
    }

    println!("{}", dp[n][0]);
}
