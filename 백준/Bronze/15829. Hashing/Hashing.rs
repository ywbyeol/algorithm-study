use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let (mut h, mut r) = (0, 1);
    for c in s.split_whitespace().skip(1).next().unwrap().chars() {
        h = (h + (c as u64 - 96) * r) % 1234567891;
        r = (r * 31) % 1234567891;
    }
    println!("{}", h);
}