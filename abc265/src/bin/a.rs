fn main() {
    proconio::input! {
        x: i32,
        y: i32,
        n: i32,
    }
    let a = x * n;
    let b = (n/3) * y + (n%3) * x;

    println!("{}", a.min(b));
}