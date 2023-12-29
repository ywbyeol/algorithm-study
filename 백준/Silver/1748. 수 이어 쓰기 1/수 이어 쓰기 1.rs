fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let len = s.trim().len() as u32;
    let mut r = (1..len).fold(0, |a, i| a + 9 * 10u64.pow(i - 1) * i as u64);
    r += (s.trim().parse::<u64>().unwrap() - 10u64.pow(len - 1) + 1) * len as u64;
    print!("{}", r);
}