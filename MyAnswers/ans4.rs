use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{h: usize, w: usize, nums: [[usize; w]; h]}
    let mut rows: Vec<_> = Vec::new();
    let mut columns: Vec<_> = Vec::new();
    
    // row
    for i in 0..h {
        let mut sum: usize = 0;
        for j in 0..w {
            sum += nums[i][j];
        }
        rows.push(sum);
    }
    // column
    for j in 0..w {
        let mut sum: usize = 0;
        for i in 0..h {
            sum += nums[i][j];
        }
        columns.push(sum);
    }

    for i in 0..h {
        for j in 0..w {
            if j == w - 1 {
                println!("{}", rows[i] + columns[j] - nums[i][j]);
            } else {
                print!("{} ", rows[i] + columns[j] - nums[i][j]);
            }
        }
    }
}
