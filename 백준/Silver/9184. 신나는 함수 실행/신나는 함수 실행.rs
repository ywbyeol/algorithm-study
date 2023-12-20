use std::collections::HashMap;

fn main() {
    let mut m = HashMap::<(_, _, _), _>::new();
    for l in std::io::stdin().lines().flatten() {
        let l = Vec::from_iter(l.split_whitespace().flat_map(str::parse));
        if l[0] == -1 && l[1] == -1 && l[2] == -1 {
            return;
        }
        let r = w(l[0], l[1], l[2], &mut m);
        println!("w({}, {}, {}) = {}", l[0], l[1], l[2], r);
    }
}

fn w(a: i32, b: i32, c: i32, m: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    let r = if let Some(&r) = m.get(&(a, b, c)) {
        r
    } else if a <= 0 || b <= 0 || c <= 0 {
        1
    } else if a > 20 || b > 20 || c > 20 {
        w(20, 20, 20, m)
    } else if a < b && b < c {
        w(a, b, c - 1, m) + w(a, b - 1, c - 1, m) - w(a, b - 1, c, m)
    } else {
        w(a - 1, b, c, m) + w(a - 1, b - 1, c, m) + w(a - 1, b, c - 1, m)
            - w(a - 1, b - 1, c - 1, m)
    };
    m.insert((a, b, c), r);
    r
}