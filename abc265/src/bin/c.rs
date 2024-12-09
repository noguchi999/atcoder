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

    let visited = vec![vec![false; w]; h];
    let mut now_g = 0;
    let mut now_r = 0;

    for i in 0..(10_i32.pow(10)) {
        if visited[now_g][now_r] == 1 {
            println!("-");
            return;
        }
    }
}
