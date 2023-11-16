fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    print!("{}", (1..=n.trim().parse().unwrap()).filter(f).count());
}

fn f(i: &u16) -> bool {
    let (mut i, mut d) = (*i, i % 10);
    i /= 10;
    d -= i % 10;
    while i >= 10 {
        let l = i % 10;
        i /= 10;
        if l - i % 10 != d {
            return false;
        }
    }
    true
}