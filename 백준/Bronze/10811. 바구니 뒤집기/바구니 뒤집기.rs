fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = s.split_whitespace().map(|v| v.parse::<usize>().unwrap());
    let (mut v, _) = ((1..=i.next().unwrap()).collect::<Vec<_>>(), i.next());
    for n in Vec::from_iter(i).chunks(2) {
        v[n[0] - 1..n[1]].reverse()
    }
    v.iter().for_each(|n| print!("{} ", n));
}