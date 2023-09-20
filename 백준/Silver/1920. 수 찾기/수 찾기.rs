use std::io::{self, Read, Write};

fn main() {
    let mut i = String::new();
    io::stdin().read_to_string(&mut i).unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let l: Vec<&str> = i.lines().collect();
    let mut b = f(l[1]);
    b.sort();
    let d = f(l[3]);
    for n in d {
        writeln!(o, "{}", s(&b, &n)).unwrap();
    }
}

fn f(l: &str) -> Vec<isize> {
    return l.split_whitespace().map(|x| x.parse().unwrap()).collect();
}

fn s(v: &Vec<isize>, t: &isize) -> i32 {
    let mut l = 0;
    let mut r = v.len() as isize - 1;
    while l <= r {
        let m = (l + r) / 2;
        if v[m as usize] == *t {
            return 1;
        } else if v[m as usize] < *t {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    0
}