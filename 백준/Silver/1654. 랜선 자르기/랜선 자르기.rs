fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().skip(1).flat_map(str::parse);
    let (n, v) = (i.next().unwrap(), Vec::from_iter(i));
    let s: u64 = v.iter().sum();
    let (mut l, mut r, mut a) = (1, s / n + 1, 0);
    while l < r {
        let m = l + (r - l) / 2;
        if v.iter().map(|&i| i % m).sum::<u64>() > s - m * n {
            r = m;
            continue;
        }
        (a = m, l = m + 1);
    }
    print!("{}", a);
}