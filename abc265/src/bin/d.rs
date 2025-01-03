use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i32,
        q: i32,
        r: i32,
        a: [i32; n],
    };

    let mut p_list: Vec<(usize, usize)> = Vec::new();
    let mut s = a[0];
    let mut r = 0;

    for l in 0..n {
        while r < n {
            if s < p {
                r += 1;
                if r == n {
                    break;
                }
                s += a[r];
            } else {
                if s == p {
                    p_list.push((l, r));
                }
                break;
            }
        }
        s -= a[l];
    }

    let mut q_list: Vec<(usize, usize)> = Vec::new();
    let mut s = a[0];
    let mut r = 0;
    for l in 0..n {
        while r < n {
            if s < q {
                r += 1;
                if r == n {
                    break;
                }
                s += a[r];
            } else {
                if s == q {
                    q_list.push((l, r));
                }
                break;
            }
        }
        s -= a[l];
    }

    let mut r_list: Vec<(usize, usize)> = Vec::new();
    let mut s = a[0];
    let mut r = 0;
    for l in 0..n {
        while r < n {
            if s < r {
                r += 1;
                if r == n {
                    break;
                }
                s += a[r];
            } else {
                if s == r {
                    r_list.push((l, r));
                }
                break;
            }
        }
        s -= a[l];
    }

    if p_list.is_empty() || q_list.is_empty() || r_list.is_empty() {
        println!("No");
        std::process::exit(0);
    }
}
