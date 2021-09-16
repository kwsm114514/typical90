use proconio::{input, fastout};
use std::f64::consts::PI;

const RAD_TO_DEG: f64 = std::f64::consts::FRAC_1_PI * 180.0;

#[fastout]
fn main() {
    input!{t: f64, l: f64, x: f64, y: f64, q: usize, times: [f64; q]}

    // (y, z)
    let position = |time: f64| -> (f64, f64) {
        let y: f64 = - (l / 2.0) * (2.0 * PI * time / t - PI / 2.0).cos();
        let z: f64 = l / 2.0 + (l / 2.0) * (2.0 * PI * time / t - PI / 2.0).sin();
        return (y, z);
    };

    for i in 0..q {
        let (yi, zi) = position(times[i]);
        let tan: f64 = zi / (x.powi(2) + (y - yi).powi(2)).sqrt();
        let angle: f64 = tan.atan() * RAD_TO_DEG;

        println!("{}", angle);
    }
}
