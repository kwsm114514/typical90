use proconio::{input, fastout};

fn mod_pow(a: usize, mut exp: usize, m: usize) -> usize {
    let mut result = 1;
    let mut mul = a;
    while exp > 0 {
        if (exp & 1) == 1 {
            result = result * mul % m;
        }
        mul = mul * mul % m;
        exp >>= 1;
    }
    result
}

fn precompute_factorials(n: usize, m: usize) -> (Vec<usize>, Vec<usize>) {
    let mut factorials: Vec<usize> = vec![1; n + 1];
    for i in 2..=n {
        factorials[i] = factorials[i - 1] * i % m;
    }
    let finv = mod_pow(factorials[n], m - 2, m);
    let mut inversions: Vec<usize> = vec![1; n + 1];
    inversions[n] = finv;
    for i in (2..n).rev() {
        inversions[i] = inversions[i + 1] * (i + 1) % m;
    }
    (factorials, inversions)
}

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {n: usize}
    let (facts, finvs) = precompute_factorials(n, MOD);

    for k in 1..=n {
        let mut j = 1;
        let mut ans = 0;
        while n >= j + (k - 1) * (j - 1) {
            ans = (ans + facts[n - (k - 1) * (j - 1)] * finvs[j] % MOD * finvs[n - (k - 1) * (j - 1) - j] % MOD) % MOD;
            j += 1;
        }
        println!("{}", ans);
    }
}
