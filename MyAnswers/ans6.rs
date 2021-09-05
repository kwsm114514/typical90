use proconio::{input, marker::Chars};

fn main() {
    input!{n: usize, k: usize, s: Chars}
    let length: usize = s.len();
    // next[i][j]: 左からi文字目のアルファベットより右にあるものでもっとも左のindex
    let mut next: Vec<Vec<usize>> = vec![vec![length; 26]; length + 1];
    // 貪欲法の前計算をする。
    for i in (0..length).rev() {
        for j in 0..26 {
            // s[i]とj番目のアルファベットが一致したら
            if (s[i] as usize - 'a' as usize) == j {
                next[i][j] = i;
            } else {
                next[i][j] = next[i + 1][j];
            }
        }
    }

    // 貪欲法の計算
    let mut ans: String = String::new();
    let mut cur: usize = 0;
    // 採用した文字数の合計はi
    for i in 1..=k {
        for j in 0..26 {
            // aにちかいアルファベットから検討する。
            let next_position: usize = next[cur][j];
            let possible_length: usize = length - next_position - 1 + i;
            if possible_length >= k {
                ans = format!("{}{}", ans, (('a' as u8 + j as u8) as char).to_string());
                cur = next_position + 1;
              	break;
            }
        }
    }

    println!("{}", ans);
}
