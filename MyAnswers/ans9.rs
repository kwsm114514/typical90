use proconio::input;
use std::f64::consts::PI;

const PI_TO_DEG: f64 = 180f64 / PI;

// 数学のベクトル
#[derive(Debug, Clone)]
struct Vector {
    x: f64, y: f64, length: f64
}

impl Vector {
    fn new(x: f64, y: f64) -> Self {
        Self{x: x, y: y, length: (x.powi(2) + y.powi(2)).sqrt()}
    }

    fn zero() -> Self {
        Self{x: 0f64, y: 0f64, length: 0f64}
    }

    fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y;
    }

    fn angle(&self, other: &Self) -> f64 {
        let cos: f64 = self.dot(other) / (self.length * other.length);
        return cos.acos() * PI_TO_DEG;
    }
}

fn main() {
    input!{n: usize, xy: [(f64, f64); n]}
    // vectors[i][j] iからjのベクトル
    let mut vectors: Vec<Vec<_>> = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                vectors[i].push(Vector::zero());
            } else {
                vectors[i].push(Vector::new(xy[j].0 - xy[i].0, xy[j].1 - xy[i].1));
            }
        }
    }
    let mut ans: f64 = 0f64;

    // iがベクトルの始点
    // 全探索
    // 二分探索をする法が速い。
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i != j && i != k && j != k {
                    let degree: f64 = vectors[i][j].angle(&vectors[i][k]);
                    if ans < degree {
                        ans = degree;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
// TLE
