fn main() {
    let s = std::io::stdin().lines().map(|n| n.unwrap()).enumerate();
    let t = s.map(|(i, n)| n.parse::<u32>().unwrap() * if i % 2 == 0 { 1 } else { 3 });
    println!("The 1-3-sum is {}", 91 + t.sum::<u32>())
}