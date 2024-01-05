fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let a = Vec::from_iter(s.split_whitespace().flat_map(str::parse).skip(1));
    let mut b = a.clone();
    b.sort_unstable();
    for i in a {
        let i = b.iter().position(|&x| x == i).unwrap();
        print!("{} ", i);
        b[i] = -1;
    }
}