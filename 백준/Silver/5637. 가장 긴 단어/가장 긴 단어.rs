fn main() {
    let l = std::io::read_to_string(std::io::stdin()).unwrap();
    let i = String::from_iter(l.trim().chars().map(|c| {
        if c.is_ascii_alphabetic() || c == '-' {
            c
        } else {
            ' '
        }
    }));
    let i = i.split_at(i.len() - 5).0;
    let mut i = i.split_whitespace();
    let l = i.clone().max_by_key(|&w| w.len()).unwrap().len();
    print!("{}", i.find(|v| v.len() == l).unwrap().to_lowercase());
}