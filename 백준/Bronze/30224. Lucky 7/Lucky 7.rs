fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let a = match (!s.contains("7"), s.trim().parse::<u32>().unwrap() % 7 != 0) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    println!("{}", a);
}