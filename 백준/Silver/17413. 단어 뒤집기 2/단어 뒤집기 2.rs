fn main() {
    let w = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut w = Vec::from_iter(w.trim().chars());
    let mut i = 0;
    while i < w.len() {
        if w[i] == '<' {
            let mut t = w.iter().skip(i + 1);
            i = t.position(|&x| x == '>').map_or(w.len(), |x| x + i + 2);
        } else if w[i].is_alphanumeric() {
            let s = i;
            while i < w.len() && w[i].is_alphanumeric() {
                i += 1;
            }
            w[s..i].reverse();
        } else {
            i += 1;
        }
    }
    print!("{}", String::from_iter(w));
}