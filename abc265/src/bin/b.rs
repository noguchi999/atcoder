fn main() {
    proconio::input! {
        n: i32,
        m: i32,
        t: i32,
    };

    let mut a = [0; 3];
    let mut bonus = [0; n + 1];
    for _ in 0..m {
        proconio::input! {
            x: i32,
            y: i32,
        };
        bonus[x] = y;
    }
}
