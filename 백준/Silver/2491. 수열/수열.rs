fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s: Vec<u32> = s.split_whitespace().flat_map(str::parse).skip(1).collect();
    let (mut a, mut d, mut m) = (1, 1, 1);
    for i in 1..s.len() {
        a = if s[i] >= s[i - 1] { a + 1 } else { 1 };
        d = if s[i] <= s[i - 1] { d + 1 } else { 1 };
        m = m.max(a).max(d);
    }
    print!("{}", m);
}