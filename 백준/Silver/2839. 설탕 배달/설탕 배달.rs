fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (mut n, mut c) = (s.trim().parse::<i16>().unwrap(), 0);
    while n % 5 != 0 && n > 2 {
        n -= 3;
        c += 1;
    }
    print!("{}", if n % 5 == 0 { c + n / 5 } else { -1 });
}