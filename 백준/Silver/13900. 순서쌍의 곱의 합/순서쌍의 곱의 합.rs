fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let (n, mut v) = (s.next().unwrap(), vec![0]);
    (0..n).for_each(|i| v.push(v[i] + s.next().unwrap()));
    let r = (0..n).fold(0, |a, i| a + (v[i + 1] - v[i]) * (v[n] - v[i + 1]));
    print!("{}", r);
}