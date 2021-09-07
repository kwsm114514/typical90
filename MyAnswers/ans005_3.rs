use proconio::input;
const MOD: usize = 1e9 as usize + 7;

fn main() {
    // n: 桁数、b: 倍数、k: 使える数字の個数、nums: 使える数字
    input!{n: usize, b: usize, k: usize, nums: [usize; k]}

    // 10の2のi乗の前計算をする。
    let mut power10: Vec<usize> = vec![0; 64];
    for i in 0..63 {
        power10[i] = modpow(10, 1 << i, b);
    }

    // dp[i][j] i: 上から何桁目か　j:bで割ったときのあまり
    let mut dp: Vec<Vec<usize>> = vec![vec![0; b]; 64];
    // dpの初期化
    for i in 0..k {
        dp[0][nums[i] % b] += 1;
    }
    // dp[1][i], dp[2][i], ..., dp[2^n][i] を求める
    for i in 0..63 {
        for j in 0..b {
            for k in 0..b {
                // power10 index: 0..63
                let next: usize = (j * power10[i] + k) % b;
                dp[i + 1][next] += dp[i][j] * dp[i][k];
                dp[i + 1][next] %= MOD;
            }
        }
    }

    // 繰り返し二乗法により dp[N][i] を求める
    let mut answers: Vec<Vec<usize>> = vec![vec![0; b]; 64];
    answers[0][0] = 1;
    for i in 0..63 {
        if (n >> i) & 1 == 1 {
            for j in 0..b {
                for k in 0..b {
                    let next: usize = (j * power10[i] + k) % b;
                    answers[i + 1][next] += answers[i][j] * dp[i][k];
                    answers[i + 1][next] %= MOD; 
                }
            }
        } else {
            for j in 0..b {
                answers[i + 1][j] = answers[i][j];
            }
        }
    }

    println!("{}", answers[63][0] % MOD);
}

fn modpow(mut base: usize, ruijou: usize, modu: usize) -> usize {
    let mut ans: usize = 1;
    for i in 0..63 {
        if (ruijou >> i) & 1 == 1 {
            ans *= base;
            ans %= modu;
        }
        // baseが倍々になる。
        base = base.pow(2);
        base %= modu;
    }
    return ans;
}
