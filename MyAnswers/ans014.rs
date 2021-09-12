use proconio::input;

// 貪欲法
fn main() {
    input!{n: usize, mut students: [isize; n], mut schools: [isize; n]}
    // 距離の小さい順に並び変える。
    students.sort();
    schools.sort();

    let mut ans: isize = 0;
    for i in 0..n {
        ans += (students[i] - schools[i]).abs();
    }

    println!("{}", ans);
}
