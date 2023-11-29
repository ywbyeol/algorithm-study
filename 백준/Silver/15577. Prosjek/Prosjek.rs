fn main() {
    let i = std::io::stdin().lines().skip(1);
    let mut i: Vec<i32> = i.flat_map(|l| l.unwrap().parse()).collect();
    i.sort();
    let i = i.into_iter().map(|l| l as f32);
    println!("{}", i.reduce(|a, e| (a + e) / 2.0).unwrap());
}