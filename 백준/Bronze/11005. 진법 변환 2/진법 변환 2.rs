fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (n, b) = s.split_once(" ").unwrap();
    let f = |s: &str| s.trim().parse::<u32>().unwrap();
    let (mut n, b, mut v) = (f(n), f(b), String::new());
    while n != 0 {
        if n % b > 9 {
            v.push((n % b + 55) as u8 as char)
        } else {
            v.push((n % b + 48) as u8 as char)
        };
        n /= b;
    }
    println!("{}", v.chars().rev().collect::<String>());
}