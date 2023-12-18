fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().flat_map(str::parse);
    let (n, m, v) = (i.next().unwrap(), i.next().unwrap(), Vec::from_iter(i));
    let (mut l, mut r, mut s, mut c) = (0, 0, 0, 0);
    while r <= n as usize {
        if s < m {
            s += v.get(r).unwrap_or(&0);
            r += 1;
        } else {
            c += (s == m) as u32;
            s -= v[l];
            l += 1;
        }
    }
    print!("{}", c);
}