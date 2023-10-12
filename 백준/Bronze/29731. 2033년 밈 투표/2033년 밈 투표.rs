const R: [&str; 7] = [
    "give you up",
    "let you down",
    "run around and desert you",
    "make you cry",
    "say goodbye",
    "tell a lie and hurt you",
    "stop",
];

fn main() {
    let r: Vec<_> = R.iter().map(|l| format!("Never gonna {}", l)).collect();
    let i = std::io::stdin().lines().skip(1).map(|v| v.unwrap());
    let mut b = false;
    for l in i {
        if !r.contains(&l) {
            b = true;
        }
    }
    println!("{}", if b { "Yes" } else { "No" });
}