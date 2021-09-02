use proconio::input;

// bitで全探索する。
fn main() {
    input!{n: usize}
    let mut answers = std::collections::BTreeSet::new();

    for bit in 0..(1 << n) {
        if ok(bit, n) {
            answers.insert(out(bit, n));
        }
    }

    for ans in answers {
        println!("{}", ans);
    }
}

// 0が(, 1が)を表す。
fn ok(bit: usize, n: usize) -> bool {
    // 負の値になる場合はisizeをつかう。
    let mut is_ok: isize = 0;
    for i in 0..n {
        if is_ok < 0 {
            return false;
        }
        if (bit >> i) & 1 == 0 {
            is_ok += 1;
        } else {
            is_ok -= 1;
        }
    }
    return is_ok == 0;
}

fn out(bit: usize, n: usize) -> String {
    let mut ans = String::new();
    for i in 0..n {
        if (bit >> i) & 1 == 0 {
            ans += "("
        } else {
            ans += ")"
        }
    }
    return ans;
}
