fn main() {
    let l = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut l = l.split_whitespace().flat_map(str::parse::<i64>);
    let mut r = || l.next().unwrap();
    let (n, k, mut v) = (r() as usize, r() as usize, vec![0, r()]);
    (1..n).for_each(|i| v.push(v[i] + r()));
    print!("{}", (k..=n).map(|i| v[i] - v[i - k]).max().unwrap_or(v[n]));
}