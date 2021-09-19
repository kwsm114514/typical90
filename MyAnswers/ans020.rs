use proconio::input;

// floatだと誤差がでる。
fn main() {
    input!{a: usize, b: u32, c: usize}

    if a < c.pow(b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
