#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        r: usize,
        x: usize,
        y: usize
    }
    // 頂点からの距離 / 半径
    let distance = (x.pow(2) + y.pow(2)) as f64;
    let radius = r.pow(2) as f64;
    println!("{}", ((distance / radius) as f64).sqrt().ceil() as usize);
}
