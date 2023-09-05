fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut a: Vec<String> = Vec::new();
    for i in 0..s.trim().len() {
        a.push(i.to_string());
    }
    println!("{}", a.join("\n"));
}