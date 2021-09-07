use proconio::input;
const MOD: usize = 1e9 as usize + 7;

fn main() {
    // n: 桁数、b: 倍数、k: 使える数字の個数、nums: 使える数字
    input!{n: usize, b: usize, k: usize, nums: [usize; k]}

    // 変な行列をつくる。隣接行列？
    let mut mat = SquareMatrix::new(b);
    for i in 0..b {
        for j in 0..k {
            let next: usize = (10 * i + nums[j]) % b;
            mat.matrix[i][next] += 1;
        }
    }

    // 行列をｎ乗して答えを出す。
    let ans = mat.pow(n);

    println!("{}", ans.matrix[0][0] % MOD);
}

// isizeにも適用できるようにジェネリクスを後で追加したい。
#[derive(Debug, Clone)]
struct SquareMatrix {
    size: usize, matrix: Vec<Vec<usize>>
}

impl SquareMatrix {
    fn new(size: usize) -> Self {
        Self{size: size, matrix: vec![vec![0; size]; size]}
    }

    fn multiply(&self, b: &Self) -> Self {
        let mut a_dot_b = Self::new(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                for k in 0..self.size {
                    a_dot_b.matrix[i][k] += self.matrix[i][j] * b.matrix[j][k];
                    // MODは要らない場合もある。
                    a_dot_b.matrix[i][k] %= MOD
                }
            }
        }
        return a_dot_b;
    }

    fn pow(&self, t: usize) -> Self {
        let mut matrixes: Vec<_> = Vec::new();
        // メモ化
        // OK: self.clone() NG: self
        matrixes.push(self.clone());
        for i in 1..64 {
            matrixes.push(matrixes[i - 1].multiply(&matrixes[i - 1]));
        } 

        let mut identity = Self::new(self.size);
        for i in 0..self.size {
            identity.matrix[i][i] = 1;
        }

        for i in (0..64).rev() {
            if (t >> i) & 1 == 1 {
                identity = identity.multiply(&matrixes[i]);
            }
        }

        return identity;
    }
}
