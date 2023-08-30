fn main() {
    let mut i = String::new();
    std::io::stdin().read_line(&mut i).unwrap();
    let h = i.matches(":-)").count();
    let s = i.matches(":-(").count();
    if h == 0 && s == 0 {
        println!("none");
    } else if h == s {
        println!("unsure");
    } else if h > s {
        println!("happy");
    } else if h < s {
        println!("sad");
    }
}