fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s = s.split_whitespace().skip(1).flat_map(str::parse);
    let t: Vec<u8> = s.collect();
    let f = || t.chunks(2).map(|t| (t[0], t[1]));
    let r = f().map(|p| f().filter(|&v| p.0 < v.0 && p.1 < v.1).count() + 1);
    r.for_each(|v| print!("{} ", v));
}