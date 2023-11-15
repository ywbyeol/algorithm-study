fn main() {
    let mut l = std::io::stdin().lines().flatten();
    let (v, a) = (l.next().unwrap(), Vec::from_iter("WB".repeat(4).chars()));
    let v = Vec::from_iter(v.split(' ').flat_map(str::parse));
    let (b, mut r) = (Vec::from_iter(a.iter().rev().cloned()), 32);
    let c = Vec::from_iter((1..9).map(|i| if i % 2 == 1 { &a } else { &b }.clone()));
    let b = Vec::from_iter(l.take(v[0]).map(|l| l.chars().collect()));
    (0..(v[0] - 7)).for_each(|i| (0..(v[1] - 7)).for_each(|j| r = r.min(f(&c, &b, i, j))));
    println!("{}", r);
}

fn f(c: &[Vec<char>], b: &[Vec<char>], x: usize, y: usize) -> u8 {
    let mut r = 0;
    (0..8).for_each(|i| (0..8).for_each(|j| r += if c[i][j] == b[x + i][y + j] { 1 } else { 0 }));
    r.min(64 - r)
}