use proconio::input;

fn main() {
    input!{h: usize, w: usize}
    let new_h: usize = if h % 2 == 0 {
        h / 2
    } else {
        h / 2 + 1
    };
    let new_w: usize = if w % 2 == 0 {
        w / 2
    } else {
        w / 2 + 1
    };
    // 境界条件に気を付ける。
    let ans: usize = if h == 1 || w == 1 {
        h * w
    } else {
        new_h * new_w
    };

    println!("{}", ans);
}
