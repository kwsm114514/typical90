use proconio::input;

fn main() {
    input!{n: usize, k: i64, av: [i64; n], bv: [i64; n]}
    let mut difsum: i64 = 0;
    for i in 0..n {
        difsum += (av[i] - bv[i]).abs();
    }
    // iterator
    // let difsum = av.iter().zip(bv.iter())
    //     .map(|(&x, &y)| (x - y).abs())
    //     .sum::<i64>();

    if (difsum - k).abs() % 2 == 0 && (difsum - k) <= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
