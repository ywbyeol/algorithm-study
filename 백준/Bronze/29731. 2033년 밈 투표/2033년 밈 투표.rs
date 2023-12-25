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
    let r = Vec::from_iter(R.iter().map(|l| format!("Never gonna {}", l)));
    let mut i = std::io::stdin().lines().skip(1).flatten();
    let b = i.all(|l| r.contains(&l));
    print!("{}", if b { "No" } else { "Yes" });
}