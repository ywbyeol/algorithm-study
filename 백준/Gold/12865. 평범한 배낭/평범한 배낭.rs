fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().map(|n| n.parse().unwrap());
    let mut r = || s.next().unwrap();
    let (n, k) = (r(), r());
    let mut d = vec![vec![0; k + 1]; n + 1];
    for i in 1..=n {
        let (w, v) = (r(), r());
        for j in 0..=k {
            d[i][j] = d[i - 1][j];
            if w <= j {
                d[i][j] = d[i][j].max(d[i - 1][j - w] + v)
            }
        }
    }
    println!("{}", d[n][k]);
}