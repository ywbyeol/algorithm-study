fn main() {
    let mut s = vec![147, 159, 258, 369, 741, 840, 852, 951];
    s.extend(1..100);
    for i in 1..=9_u64 {
        let t = i * 111111111111111111;
        s.push(t);
        (1..18).for_each(|j| s.push(t / 10_u64.pow(j)));
    }
    let l = [2468, 9630, 13579, 86420, 97531, 123456789, 9876543210];
    l.iter().for_each(|n| f(&mut s, *n));
    s.sort_unstable();
    s.dedup();
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    match s.binary_search(&n.trim().parse().unwrap()) {
        Ok(i) => print!("{}", i + 1),
        Err(i) => print!("{}", i),
    }
}

fn f(s: &mut Vec<u64>, n: u64) {
    for i in 1..n.to_string().len() {
        let m = n.to_string().split_at(i - 1).1.parse().unwrap();
        s.extend([n / 10_u64.pow(i as u32 - 1), m]);
        (1..m.to_string().len()).for_each(|j| s.push(m / 10_u64.pow(j as u32 - 1)));
    }
}