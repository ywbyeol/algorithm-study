fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{}", if v[0] * 2 >= v[1] { "E" } else { "H" });
}