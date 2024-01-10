fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (a, b) = s.trim().split_once(" ").unwrap();
    let r = (0..=b.len() - a.len()).map(|i| {
        let t = &b[i..i + a.len()];
        t.chars().zip(a.chars()).filter(|(x, y)| x != y).count()
    });
    print!("{}", r.min().unwrap());
}