use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n: usize, users: [String; n]}
    // hashsetで計算量を改善
    let mut hs = std::collections::HashSet::new();
    
    for i in 0..n {
        if hs.contains(&users[i]) {
            continue;
        }
        println!("{}", i + 1);
        hs.insert(&users[i]);
    }
}
