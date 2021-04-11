#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let a = n.len();
    // nからtrailinng_zeroを除き、回文が成立するかを調べる。
    let mut cnt_trailling_zero = 0;
    for i in n.iter().rev() {
        if i == &'0' {
            cnt_trailling_zero += 1;
        } else {
            break;
        }
    }
    let len = a - cnt_trailling_zero;
    for i in 0..len / 2 {
        if n[i] == n[len - 1 - i] {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
