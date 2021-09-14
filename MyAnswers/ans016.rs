use proconio::input;

fn main() {
    input!{n: usize, mut abc: [usize; 3]}
    // sortで貪欲法の準備をする。
    abc.sort();
    let mut ans: usize = std::usize::MAX;

    // 全探索
    let max_i: usize = n / abc[2];
    for i in (0..=max_i).rev() {
        let max_j: usize = (n - abc[2] * i) / abc[1];
        for j in (0..=max_j).rev() {
            if (n - abc[2] * i - abc[1] * j) % abc[0] == 0 {
                let koho: usize = i + j + (n - abc[2] * i - abc[1] * j) / abc[0];
                if ans > koho {
                    ans = koho;
                }
            }
        }
    }

    println!("{}", ans);
}
