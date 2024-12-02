fn main() {
    proconio::input! {
        h: i32,
        w: i32,
    };
    let mut g = vec![];

    for i in 0..h {
        proconio::input! {
            s: String,
        };
        g.push(s);
    }
}
