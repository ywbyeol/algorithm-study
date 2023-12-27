use std::io::{self, Write};

fn main() {
    let (s, mut p) = (io::read_to_string(io::stdin()).unwrap(), vec![true; 246913]);
    (p[0] = false, p[1] = false, p[246912] = false);
    for i in 2..=496 {
        if p[i] {
            let mut j = i * i;
            while j < 246912 {
                p[j] = false;
                j += i;
            }
        }
    }
    let mut o = io::BufWriter::new(io::stdout());
    for n in s.lines().flat_map(str::parse).filter(|n: &usize| *n != 0) {
        writeln!(o, "{}", ((n + 1)..=(n * 2)).filter(|i| p[*i]).count()).unwrap();
    }
}