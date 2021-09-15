use proconio::{input, derive_readable};

#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Line {
    start: usize, end: usize
}

fn main() {
    input!{n: usize, m: usize, mut lines: [Line; m]}
    lines.sort_by(|a, b| a.start.cmp(&b.start));
    let mut ans: usize = 0;

    for i in 0..m {
        for j in (i+1)..m {
            let is_start_ok: bool = lines[i].start < lines[j].start && lines[j].start < lines[i].end;
            let is_end_ok: bool = lines[i].end < lines[j].end || lines[j].end < lines[i].start;
            if is_start_ok && is_end_ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
