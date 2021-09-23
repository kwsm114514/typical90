use proconio::input;

const PRIMES: [i64; 4] = [2, 3, 5, 7];

// f(x) = (x の各位の数字の積)
// m - f(m) = b を満たす　1 <= m <= n の数
fn main() {
    input!{n: i64, b: i64}

    // dfs(初めの数1, PRIMESのindex, 終わりの数n, b) = mの数
    let ans = dfs(1, 0, n, b) + if check(0, n, b) { 1 } else { 0 };
    println!("{}", ans);
}

// m - f(m) = b を満たすか?
fn check(f: i64, lim: i64, b: i64) -> bool {
    let mut m = f + b;
    if m > lim { 
        return false; 
    }
    let mut c = 1;
    while m > 0 {
        c *= m % 10;
        m /= 10;
    }
    c == f
}

// dfs(初めの数1, PRIMESのindex, 終わりの数n, b) = mの数
fn dfs(mut cur: i64, idx: usize, lim: i64, b: i64) -> usize {
    // 境界の設定
    if idx == 4 {
        return if check(cur, lim, b) { 1 } else { 0 }
    }
    let mut res = 0;
    while cur <= lim {
        res += dfs(cur, idx + 1, lim, b);
        cur *= PRIMES[idx];
    }
    res
}
