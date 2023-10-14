fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = s.split_whitespace().map(|v| v.parse().unwrap());
    let (mut v, _) = (vec![0; i.next().unwrap()], i.next());
    Vec::from_iter(i)
        .chunks(3)
        .for_each(|n| (n[0] - 1..n[1]).for_each(|i| v[i] = n[2]));
    v.iter().for_each(|n| print!("{} ", n));
}