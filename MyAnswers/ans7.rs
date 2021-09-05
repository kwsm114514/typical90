use proconio::{input, fastout};
use superslice::Ext;
const MAX: isize = std::isize::MAX;

#[fastout]
fn main() {
    input!{n: usize, mut a_rates: [isize; n], q: usize, b_rates: [isize; q]}
    a_rates.sort();
    let difs: Vec<isize> = vec![-1, 0, 1];

    // 二分探索をする。
    for i in 0..q {
        let mut ans: isize = MAX;
        let index: usize = a_rates.lower_bound(&b_rates[i]);
        for &dif in difs.iter() {
            if 0 <= index as isize + dif && index as isize + dif < n as isize {
                if ans > (a_rates[index + dif as usize] - b_rates[i]).abs() {
                    ans = (a_rates[index + dif as usize] - b_rates[i]).abs();
                }
            }
        }

        println!("{}", ans);
    }
}
