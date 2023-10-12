fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = s.split_whitespace().skip(1);
    let k: usize = i.next().unwrap().parse().unwrap();
    let mut v = Vec::from_iter(i.map(|c| c.parse::<i32>().unwrap()));
    v.sort_unstable();
    println!("{}", v[k - 1]);
}