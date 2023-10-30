fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().map(|n| n.parse().unwrap());
    let mut r = || s.next().unwrap();
    let (n, k) = (r(), r());
    let mut d = vec![0; k + 1];
    for _ in 0..n {
        let (w, v) = (r(), r());
        if w > k {
            continue;
        }
        for j in (0..=k - w).rev() {
            if d[j] != 0 {
                d[j + w] = d[j + w].max(d[j] + v);
            }
        }
        d[w] = d[w].max(v);
    }
    println!("{}", d.into_iter().fold(0, |a, b| a.max(b)));
}