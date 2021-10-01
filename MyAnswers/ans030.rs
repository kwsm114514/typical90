use proconio::input;

fn main() {
    input!{n: usize, k: usize}
    let nums_of_prime_factors: Vec<usize> = eratosthenes_factor(n);
    let ans: usize = nums_of_prime_factors.iter().filter(|&x| *x >= k).count();
    println!("{}", ans);
}

/// エラトステネスの篩 Sieve of Eratosthenes で 2からN までの素因数の種類数をカウントする
fn eratosthenes_factor(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = vec![0; n + 1];
    for current in 2..=n {
        // res[num] == 0 なら素数
        if res[current] > 0 {
            continue;
        }
        let mut i = 1;
        loop {
            let multi = current * i;
            if multi > n {
                break;
            }
            res[multi] += 1;
            i += 1;
        }
    }
    res
}
