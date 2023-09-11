fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    print!("{}", v[2] / (v[0] * 2) * v[1]);
}