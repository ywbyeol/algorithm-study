fn main() {
    std::io::stdin().lines().skip(1).for_each(|l| f(l.unwrap()));
}

fn f(cent: String) {
    let mut c: u32 = cent.parse().unwrap();
    let q = c / 25;
    c %= 25;
    let d = c / 10;
    c %= 10;
    println!("{} {} {} {}", q, d, c / 5, c % 5);
}