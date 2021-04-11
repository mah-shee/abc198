#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    if n <= 1 {
        println!("0");
    } else {
        println!("{}", n - 1);
    }
}
