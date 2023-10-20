fn main() {
    for l in std::io::stdin().lines().skip(1) {
        let v: Vec<i32> = l.unwrap().split(" ").map(|n| n.parse().unwrap()).collect();
        let mut m = 1;
        (0..v.len()).for_each(|i| ((i + 1)..v.len()).for_each(|j| m = m.max(g(v[i], v[j]))));
        println!("{}", m);
    }
}

fn g(x: i32, y: i32) -> i32 {
    return if y == 0 { x } else { g(y, x % y) };
}