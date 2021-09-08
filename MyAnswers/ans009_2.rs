use ordered_float::NotNan;
use proconio::input;
use superslice::Ext;

const RAD_TO_DEG: f64 = 180f64 / std::f64::consts::PI;

fn main() {
    input! { n: usize, points: [(f64, f64); n] }
 
    let mut minimum = NotNan::new(std::f64::consts::PI).unwrap();
    for &p1 in points.iter() {
        let mut slopes = vec![];
        for &p2 in points.iter() {
            if p1 == p2 {
                continue;
            }
            // 角度を出す。
            slopes.push(NotNan::new((p2.1 - p1.1).atan2(p2.0 - p1.0)).unwrap());
        }
        // 二分探索の準備をする。
        slopes.sort();
 
        for &slope in slopes.iter() {
            let target = -slope.signum() * (std::f64::consts::PI - slope.abs());
            // 二分探索
            let lb = slopes.lower_bound(&NotNan::new(target).unwrap());
            // 例外処理
            minimum = minimum.min(if lb >= slopes.len() || lb == 0 {
                convex_angle((*slopes.last().unwrap() - target).abs())
                    .min((slopes[0] - target).abs())
                    .into()
            // 通常の処理
            } else {
                (NotNan::new((slopes[lb] - target).abs()).unwrap())
                    .min((slopes[lb - 1] - target).abs().into())
            });
        }
    }

    println!("{}",(std::f64::consts::PI - minimum.into_inner()) * RAD_TO_DEG);
}

fn convex_angle(angle: f64) -> f64 {
    if angle >= std::f64::consts::PI {
        return 2.0 * std::f64::consts::PI - angle;
    } else {
        return angle;
    }
}
