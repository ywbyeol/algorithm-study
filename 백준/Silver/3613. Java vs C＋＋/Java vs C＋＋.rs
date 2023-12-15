use std::io;

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap().trim().to_owned();
    let (f, l) = (s.chars().next().unwrap(), s.chars().last().unwrap() == '_');
    let r = if (f.is_ascii_uppercase() || f == '_' || l || s.contains("__"))
        || s.contains('_') && s.chars().any(|c| c.is_ascii_uppercase())
    {
        "Error!".to_owned()
    } else if s.contains('_') {
        a(&s)
    } else {
        b(&s)
    };
    print!("{}", r);
}

fn a(s: &str) -> String {
    let s = s.split('_').enumerate();
    s.fold(String::new(), |mut a, (i, c)| {
        if i > 0 {
            a.push_str(&c[..1].to_ascii_uppercase());
            a.push_str(&c[1..]);
        } else {
            a.push_str(c);
        }
        a
    })
}

fn b(s: &str) -> String {
    let mut r = String::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            r.push('_');
            r.push(c.to_ascii_lowercase());
        } else {
            r.push(c);
        }
    }
    r
}