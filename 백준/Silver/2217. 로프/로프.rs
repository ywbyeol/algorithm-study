fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s: Vec<i32> = s.split_whitespace().skip(1).flat_map(str::parse).collect();
    s.sort_unstable_by(|a, b| b.cmp(a));
    let r = s.iter().enumerate().map(|(i, n)| n * (i as i32 + 1));
    println!("{}", r.max().unwrap());
}