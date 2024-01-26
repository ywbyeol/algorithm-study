fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().flat_map(str::parse);
    let mut r = || i.next().unwrap();
    let (n, s, p, l) = (r(), r(), r(), Vec::from_iter(i));
    let r = l.iter().position(|&x| x <= s).unwrap_or(n) + 1;
    let o = n == p && l[n - 1] >= s;
    print!("{}", if o { -1 } else { r as i32 });
}