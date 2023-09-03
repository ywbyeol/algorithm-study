use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut t = Vec::new();
    for i in (2..v.len() - 1).step_by(3) {
        t.push(match v[i] * 2126 + v[i + 1] * 7152 + v[i + 2] * 722 {
            0..=509999 => "#",
            510000..=1019999 => "o",
            1020000..=1529999 => "+",
            1530000..=2039999 => "-",
            _ => ".",
        });
    }
    let a: Vec<_> = t.chunks(v[1]).map(|m| m.join("")).collect();
    println!("{}", a.join("\n"));
}