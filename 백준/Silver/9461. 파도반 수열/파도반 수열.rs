fn main() {
    let r = |n| (4..=n).fold([1u64, 1, 1], |m, _| [m[1], m[2], (m[0] + m[1])])[2];
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    for n in i.split_whitespace().skip(1) {
        println!("{}", r(n.parse().unwrap()))
    }
}