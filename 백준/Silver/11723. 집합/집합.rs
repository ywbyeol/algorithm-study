use std::io::{self, Write};

fn main() {
    let (mut m, mut o) = (0, io::BufWriter::new(io::stdout()));
    for l in io::stdin().lines().skip(1).flatten() {
        let mut l = l.split_whitespace();
        let c = l.next().unwrap().chars().nth(1).unwrap();
        let x = l.next().unwrap_or("0").parse::<u32>().unwrap() - 1;
        match c {
            'd' => m |= 1 << x,
            'e' => m &= !(1 << x),
            'h' => writeln!(o, "{}", (m & (1 << x) != 0) as u8).unwrap(),
            'o' => {
                let x = 1 << x;
                m = if m & x != 0 { m & !x } else { m | x };
            }
            'l' => m = 1048575,
            'm' => m = 0,
            _ => (),
        }
    }
}