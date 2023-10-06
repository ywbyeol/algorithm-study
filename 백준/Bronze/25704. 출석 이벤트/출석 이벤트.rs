fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let c = vec![v[1], v[1] - 500, 9 * v[1] / 10, v[1] - 2000, 3 * v[1] / 4];
    let r = (if v[0] > 20 { 20 } else { v[0] }) / 5;
    let d = c[0..=r as usize].iter().min().unwrap();
    println!("{}", if d > &0 { d } else { &0 });
}