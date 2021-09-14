use proconio::input;

fn main() {
    input!{n: isize, a: isize, mut b: isize, mut c: isize}

    // b, cを小さい順に並び替える。
    let (b, c) = if b > c {
        (c, b)
    } else {
        (b, c)
    };

    let mut ans = 1_000_000;
    // b, cの最大公約数をだす。
    let gcd = euc(b, c);
    // bx + cy = gcd(b, c)を満たす(l, r)を求める
    let (l, r) = exeuc(b, c);

    for i in 0..10000 {
        if n < a * i {
            break;
        } else if (n - a * i) % gcd == 0 {
            if (n - a * i) % b == 0 {
                ans = std::cmp::min(ans, i + (n - a * i) / b);
            }
            if (n - a * i) % c == 0 {
                ans = std::cmp::min(ans, i + (n - a * i) / c);
            }

            let tpl = l * (n - a * i) / gcd;
            let tpr = r * (n - a * i) / gcd;
            
            let cgcd = c / gcd;
            let bgcd = b / gcd;

            let (nl, nr) = if tpl < 0 {
                ((cgcd + tpl % cgcd) % cgcd, -(-tpl + cgcd - 1) / cgcd * bgcd + tpr)
            } else {
                (tpl % cgcd, tpl / cgcd * bgcd + tpr)
            };
            if nl < 0 || nr < 0 {
                continue;
            } else {
                ans = std::cmp::min(ans, i + nl + nr);
            };
        }
    }

    println!("{}", ans);
}

// 一次不定方程式 ax + by = c が整数解をもつ条件
// a, b, c を 0　以外の整数とする。
// 一次不定方程式 ax + by = c が整数解をもつための必要十分条件は 
// cがgcd(a,b)で割り切れることである。

// 拡張ユークリッドの互除法
// ax + by = gcd(a, b)を満たす(x, y)を求めるアルゴリズムを具体的に導出する。
// このアルゴリズムを特に拡張ユークリッドの互除法という。
fn exeuc(a: isize, b: isize) -> (isize, isize) {
    if a < b {
        let (l, r) = exeuc(b, a);
        (r, l)
    } else if b == 0 {
        (1, 0)
    } else {
        // (qb + r)x + by = d　⇔　b (qx + y) + rx = d
        // bs + rt = d
        // qx + y = s, x = t　⇔　x = t, y = s − qt
        let (p, q) = exeuc(b, a % b);
        (q, p - a / b * q)
    }
}

// ユークリッドの互除法
fn euc(a: isize, b: isize) -> isize {
    if a < b {
        euc(b, a)
    } else if b == 0 {
        a
    } else {
        euc(b, a % b)
    }
}
