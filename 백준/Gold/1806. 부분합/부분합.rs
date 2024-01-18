fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().skip(1).flat_map(str::parse);
    let (s, v) = (i.next().unwrap(), Vec::from_iter(i));
    let (mut m, mut t, mut l, mut r) = (100001, 0, 0, 0);
    while r < v.len() {
        (t += v[r], r += 1);
        while t >= s {
            (t -= v[l], m = m.min(r - l), l += 1);
        }
    }
    print!("{}", if m != 100001 { m } else { 0 });
}