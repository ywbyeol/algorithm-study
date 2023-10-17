fn main() {
    let l = std::io::stdin().lines().skip(1);
    l.for_each(|l| println!("{}", l.unwrap().to_lowercase()));
}