fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let vec: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|num| num.trim().parse().unwrap())
        .collect();
    println!("{}", (vec[0] * vec[1]) - (vec[2] * vec[3] * vec[4]));
}