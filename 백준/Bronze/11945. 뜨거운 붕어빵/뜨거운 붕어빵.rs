fn main() {
    let i = std::io::stdin().lines().skip(1);
    i.for_each(|v| {
        println!("{}", v.unwrap().chars().rev().collect::<String>());
    });
}