use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<String> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for i in (1..(v[0].parse::<usize>().unwrap() * 2)).step_by(2) {
        println!("{}", v[i + 1].repeat(v[i].parse::<usize>().unwrap()));
    }
}