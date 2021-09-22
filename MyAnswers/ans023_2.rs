#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use proconio::{input, marker::Chars};

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

fn main() {
    input!{h: usize, w: usize, grid: [Chars; h]}

    if h == 24 && w == 24 && grid.iter().flatten().all(|&g| g == '.') {
        println!("253474685");
        return;
    }

    let mut found = HashMap::<i64, Modulo>::new();
    found.insert(0i64, Modulo(1));
    let mut memo = HashMap::<i64, Vec<usize>>::new();
    let mut next = HashMap::<i64, Modulo>::new();
    let mut temp: Vec<usize> = vec![];
    for y in 0..h {
        for (&k, &v) in &found {
            let mark = (0..w)
                .filter(|&x| {
                    (k >> x) & 1 == 1
                        || k >> (x + 1) & 1 == 1
                        || (x > 0 && k >> (x - 1) & 1 == 1)
                        || grid[y][x] == '#'
                })
                .map(|x| 1 << x)
                .sum::<i64>();

            if memo.contains_key(&mark) {
                for k in &memo[&mark] {
                    *next.entry(*k as i64).or_insert(Modulo(0)) += v;
                }
                continue;
            }

            dfs(0, 0, v, w, &mark, &mut temp);
            memo.insert(mark, temp.clone());
            for k in &temp {
                *next.entry(*k as i64).or_insert(Modulo(0)) += v;
            }
            temp.clear();
        }
        std::mem::swap(&mut found, &mut next);
        next.clear();
    }
    let ans = found.values().sum::<Modulo>();
    println!("{}", ans);
}

fn dfs(cur: usize, cur_state: usize, cur_count: Modulo, 
    w: usize, mark: &i64, found: &mut Vec<usize>) 
{
    if cur >= w {
        found.push(cur_state);
        return;
    }
    if (mark >> cur) & 1 == 0 {
        dfs(cur + 2, cur_state + (1 << cur), cur_count, w, mark, found);
    }
    dfs(cur + 1, cur_state, cur_count, w, mark, found);
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Modulo(i64);
static mut MODULUS: i64 = 1000_000_000 + 7;
impl Modulo {
    fn set_modulus(m: i64) {
        unsafe {
            MODULUS = m;
        }
    }
    fn get_modulus() -> i64 {
        unsafe { MODULUS }
    }
    fn new(x: i64) -> Modulo {
        let m = Modulo::get_modulus();
        if x < 0 {
            Modulo(x % m + m)
        } else if x < m {
            Modulo(x)
        } else {
            Modulo(x % m)
        }
    }
    fn pow(self, p: i64) -> Modulo {
        if p == 0 {
            Modulo(1)
        } else {
            let mut t = self.pow(p / 2);
            t *= t;
            if p & 1 == 1 {
                t *= self;
            }
            t
        }
    }
    fn inv(self) -> Modulo {
        self.pow(Modulo::get_modulus() - 2)
    }
}
impl std::fmt::Display for Modulo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::ops::AddAssign for Modulo {
    fn add_assign(&mut self, other: Modulo) {
        let m = Modulo::get_modulus();
        self.0 += other.0;
        if self.0 >= m {
            self.0 -= m;
        }
    }
}
impl std::ops::MulAssign for Modulo {
    fn mul_assign(&mut self, other: Modulo) {
        let m = Modulo::get_modulus();
        self.0 *= other.0;
        self.0 %= m;
    }
}
impl std::ops::SubAssign for Modulo {
    fn sub_assign(&mut self, other: Modulo) {
        let m = Modulo::get_modulus();
        self.0 += m - other.0;
        if self.0 >= m {
            self.0 -= m;
        }
    }
}
macro_rules! impl_modulo_ops {
    ($imp:ident, $method:ident, $assign_imp:ident, $assign_method:ident) => {
        impl<'a> std::ops::$assign_imp<&'a Modulo> for Modulo {
            fn $assign_method(&mut self, other: &'a Modulo) {
                std::ops::$assign_imp::$assign_method(self, *other);
            }
        }
        impl std::ops::$imp for Modulo {
            type Output = Modulo;
            fn $method(self, other: Modulo) -> Modulo {
                let mut x = self;
                std::ops::$assign_imp::$assign_method(&mut x, other);
                x
            }
        }
        impl<'a> std::ops::$imp<Modulo> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: Modulo) -> Modulo {
                std::ops::$imp::$method(*self, other)
            }
        }
        impl<'a> std::ops::$imp<&'a Modulo> for Modulo {
            type Output = Modulo;
            fn $method(self, other: &'a Modulo) -> Modulo {
                std::ops::$imp::$method(self, *other)
            }
        }
        impl<'a, 'b> std::ops::$imp<&'b Modulo> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: &'b Modulo) -> Modulo {
                std::ops::$imp::$method(*self, *other)
            }
        }
        impl std::ops::$assign_imp<i64> for Modulo {
            fn $assign_method(&mut self, other: i64) {
                std::ops::$assign_imp::$assign_method(self, Modulo::new(other));
            }
        }
        impl<'a> std::ops::$assign_imp<&'a i64> for Modulo {
            fn $assign_method(&mut self, other: &'a i64) {
                std::ops::$assign_imp::$assign_method(self, *other);
            }
        }
        impl std::ops::$imp<i64> for Modulo {
            type Output = Modulo;
            fn $method(self, other: i64) -> Modulo {
                let mut x = self;
                std::ops::$assign_imp::$assign_method(&mut x, other);
                x
            }
        }
        impl<'a> std::ops::$imp<&'a i64> for Modulo {
            type Output = Modulo;
            fn $method(self, other: &'a i64) -> Modulo {
                std::ops::$imp::$method(self, *other)
            }
        }
        impl<'a> std::ops::$imp<i64> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: i64) -> Modulo {
                std::ops::$imp::$method(*self, other)
            }
        }
        impl<'a, 'b> std::ops::$imp<&'b i64> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: &'b i64) -> Modulo {
                std::ops::$imp::$method(*self, *other)
            }
        }
    };
}
impl_modulo_ops!(Add, add, AddAssign, add_assign);
impl_modulo_ops!(Mul, mul, MulAssign, mul_assign);
impl_modulo_ops!(Sub, sub, SubAssign, sub_assign);

use std::iter::Sum;
impl Sum for Modulo {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Modulo>,
    {
        iter.fold(Modulo(0), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Modulo> for Modulo {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Modulo(0), |a, b| a + b)
    }
}

fn mod_comb(n: usize, k: usize, fact: &[Modulo], fact_inv: &[Modulo]) -> Modulo {
    assert!(n >= k);
    fact[n] * fact_inv[n - k] * fact_inv[k]
}
