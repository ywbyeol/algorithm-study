fn main() {
    let n: i32 = r().parse().unwrap();
    let mut d: Vec<char> = r().chars().collect();
    for _ in 0..n {
        for (i, c) in r().chars().enumerate() {
            if d[i] != c {
                d[i] = '?'
            }
        }
    }
    println!("{}", d.iter().collect::<String>());
}

fn r() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_owned()
}