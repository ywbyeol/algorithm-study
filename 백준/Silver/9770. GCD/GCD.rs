fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v: Vec<u32> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let mut m = 1;
    (0..v.len()).for_each(|i| ((i + 1)..v.len()).for_each(|j| m = m.max(g(v[i], v[j]))));
    println!("{}", m);
}

fn g(x: u32, y: u32) -> u32 {
    return if y == 0 { x } else { g(y, x % y) };
}