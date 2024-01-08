fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse::<usize>);
    let (n, m) = (s.next().unwrap(), s.next().unwrap());
    let (mut v, mut c, mut l, mut r) = (Vec::from_iter(s), 0, 0, n - 1);
    v.sort_unstable();
    while l < r {
        if v[l] + v[r] < m {
            l += 1;
        } else if v[l] + v[r] > m {
            r -= 1;
        } else {
            (c += 1, l += 1, r -= 1);
        }
    }
    print!("{}", c);
}