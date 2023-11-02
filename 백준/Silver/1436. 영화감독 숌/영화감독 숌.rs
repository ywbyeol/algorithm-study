fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = (666..2666800).filter(|v| v.to_string().contains("666"));
    println!("{}", i.nth(n.trim().parse::<usize>().unwrap() - 1).unwrap());
}