fn main() {
    let l = std::io::read_to_string(std::io::stdin()).unwrap();
    let (mut s, mut d) = (Vec::new(), 0);
    for c in l.chars() {
        if c == '*' {
            break;
        }
        if let Some(&'(') = s.last() {
            if c == ')' {
                s.pop();
                d -= 1;
                continue;
            }
        }
        s.push(c);
        d += 1;
    }
    println!("{}", d);
}