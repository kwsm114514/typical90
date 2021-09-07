use proconio::input;
// 最小値の最大化なので二分探索をする。

fn main() {
    input!{n: usize, l: usize, k: usize, kiru: [usize; n]}
    let mut possible: usize = 1;
    let mut impossible: usize = l;

    for _ in 0..100 {
        let mid: usize = (possible + impossible) / 2;
        if is_ok(n, l, k, &kiru, mid) {
            possible = mid;
        } else {
            impossible = mid;
        }
    }

    println!("{}", possible);
}

fn is_ok(n: usize, l: usize, k: usize, kiru: &Vec<usize>, mid: usize) -> bool {
    let mut cur: usize = 0;
    let mut kireme: usize = 0;

    for i in 0..=n {
        if i != n {
            if kiru[i] - cur >= mid {
                cur = kiru[i];
                kireme += 1;
            }
        }
        if i == n {
            if l - cur < mid && kireme <= k {
                return false;
            }
        }
    }
    if kireme >= k {
        return true;
    } else {
        return false;
    }
}
