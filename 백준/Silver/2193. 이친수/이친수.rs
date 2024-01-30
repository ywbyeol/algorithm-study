fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let r = (2..=n.trim().parse().unwrap()).fold([0u64, 1], |m, _| [m[1], (m[0] + m[1])])[1];
    println!("{}", r);
}