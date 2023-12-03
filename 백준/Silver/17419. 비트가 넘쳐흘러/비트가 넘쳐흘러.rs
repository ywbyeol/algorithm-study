fn main() {
    let i = std::io::stdin().lines().nth(1).unwrap().unwrap();
    print!("{}", i.chars().filter(|&c| c == '1').count());
}