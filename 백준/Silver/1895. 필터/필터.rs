fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let (r, c, t) = (s.next().unwrap(), s.next().unwrap(), Vec::from_iter(s));
    let mut v = Vec::from_iter(t.chunks(c));
    let (n, mut a) = (v.pop().unwrap()[0], Vec::new());
    for i in 0..r - 2 {
        for j in 0..c - 2 {
            let mut t = Vec::new();
            (0..3).for_each(|k| (0..3).for_each(|l| t.push(v[i + k][j + l])));
            (t.sort(), a.push(t[4]));
        }
    }
    print!("{}", a.iter().filter(|&&v| v >= n).count());
}