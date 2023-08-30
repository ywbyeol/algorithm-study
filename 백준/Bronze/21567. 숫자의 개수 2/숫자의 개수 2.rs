use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<u64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut a = vec![0; 10];
    for d in v.iter().product::<u64>().to_string().chars() {
        if d.is_ascii_digit() {
            let i = (d as u8 - b'0') as usize;
            a[i] += 1;
        }
    }
    println!(
        "{}",
        a.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}