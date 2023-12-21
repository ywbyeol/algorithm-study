fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (s, mut c, mut n) = (s.trim().parse().unwrap(), 0, 1);
    while c < s {
        c = (n * (n + 1)) / 2u64;
        if c > s {
            break;
        }
        n += 1;
    }
    print!("{}", n - 1);
}