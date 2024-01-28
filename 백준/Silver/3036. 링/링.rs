fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let i = i.split_whitespace().skip(1).flat_map(str::parse);
    let l = Vec::from_iter(i);
    for i in 1..l.len() {
        let g = g(l[0], l[i]);
        println!("{}/{}", l[0] / g, l[i] / g);
    }
}

fn g(x: i32, y: i32) -> i32 {
    return if y == 0 { x } else { g(y, x % y) };
}