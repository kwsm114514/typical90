use proconio::input;
// 全探索をつかう。再帰関数を作る。

fn main() {
    input!{n: usize}
    let ans = String::new();
    if n % 2 == 0 {
        solve(ans, n / 2, n / 2);
    } else {
        return;
    }
}

fn solve(ans: String, forw: usize, back: usize) {
    if forw == 0 && back == 0 {
        println!("{}", ans);
        return;
    } else if forw == back {
        solve(format!("{}(", ans), forw - 1, back);
    } else if forw == 0 {
        solve(format!("{})", ans), 0, back - 1);
    } else {
        solve(format!("{}(", ans), forw - 1, back);
        solve(format!("{})", ans), forw, back - 1);
    }
}
