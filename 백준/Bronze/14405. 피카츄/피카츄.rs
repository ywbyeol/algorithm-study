fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut c = i.trim().chars().peekable();
    while let Some(v) = c.next() {
        let mut f = |v| c.next() == Some(v);
        match v {
            'p' if f('i') => {}
            'k' if f('a') => {}
            'c' if f('h') && f('u') => {}
            _ => {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}