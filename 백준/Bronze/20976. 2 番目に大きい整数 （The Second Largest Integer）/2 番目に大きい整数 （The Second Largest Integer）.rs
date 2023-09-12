fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    v.sort();
    println!("{}", v[1]);
}