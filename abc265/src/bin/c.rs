use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let mut g: Vec<String> = Vec::new();

    for i in 0..h {
        input! {
            s: String,
        };
        g.push(s);
    }

    let mut visited = vec![vec![false; w]; h];
    let mut now_g = 0;
    let mut now_r = 0;

    for i in 0..(10_i32.pow(10)) {
        if visited[now_g][now_r] {
            println!("-1");
            return;
        }
        visited[now_g][now_r] = true;

        if g[now_g].chars().nth(now_r) == Some('U') {
            if now_g >= 1 {
                now_g -= 1;
            } else {
                println!("{}, {}", now_g + 1, now_r + 1);
                std::process::exit(0);
            }
        }

        if g[now_g].chars().nth(now_r) == Some('R') {
            if now_g < w - 2 {
                now_g += 1;
            } else {
                println!("{}, {}", now_g + 1, now_r + 1);
                std::process::exit(0);
            }
        }

        if g[now_g].chars().nth(now_r) == Some('D') {
            if now_g < h - 2 {
                now_g += 1;
            } else {
                println!("{}, {}", now_g + 1, now_r + 1);
                std::process::exit(0);
            }
        }
    }
}
