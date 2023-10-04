fn main() {
    let s = r();
    let mut t = s.split_whitespace();
    let s = t.next().unwrap().parse().unwrap();
    let p = t.next().unwrap().parse().unwrap();
    let d: Vec<char> = r().trim().chars().collect();
    let l: Vec<i32> = r().split_whitespace().map(|v| v.parse().unwrap()).collect();
    let (mut c, mut a) = ([0; 4], 0);
    for p in d.iter().take(p) {
        c[match p {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => unreachable!(),
        }] += 1;
    }
    if c[0] >= l[0] && c[1] >= l[1] && c[2] >= l[2] && c[3] >= l[3] {
        a += 1;
    }
    for j in p..s {
        c[match d[j - p] {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => unreachable!(),
        }] -= 1;
        c[match d[j] {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => unreachable!(),
        }] += 1;
        if c[0] >= l[0] && c[1] >= l[1] && c[2] >= l[2] && c[3] >= l[3] {
            a += 1;
        }
    }
    println!("{}", a);
}

fn r() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}