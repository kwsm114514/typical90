use proconio::input;

fn main() {
    input!{a: i64, b: i64, c: i64}
    let gcd_ab: i64 = euc(a, b);
    let gcd: i64 = euc(gcd_ab, c);
    let ans: i64 = (a / gcd - 1) + (b / gcd - 1) + (c / gcd - 1);

    println!("{}", ans);
}

fn euc(a: i64, b: i64) -> i64 {
    if a < b {
        euc(b, a)
    } else if b == 0 {
        a
    } else {
        euc(b, a % b)
    }
}
