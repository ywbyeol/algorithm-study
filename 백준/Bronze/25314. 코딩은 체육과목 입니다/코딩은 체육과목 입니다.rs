fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for _ in 0..(s.trim().parse::<i32>().unwrap() / 4) {
        print!("long ");
    }
    println!("int");
}