fn main() {
    let mut l = std::io::stdin().lines().flatten();
    let v = l.next().unwrap();
    let v = Vec::from_iter(v.split(' ').flat_map(str::parse::<usize>));
    let b = Vec::from_iter(l.map(|l| l.chars().collect()));
    let r = (0..(v[0] - 7)).flat_map(|i| (0..(v[1] - 7)).map(move |j| (i, j)));
    println!("{}", r.map(|(i, j)| f(&b, i, j)).min().unwrap());
}

fn f(b: &[Vec<char>], x: usize, y: usize) -> u8 {
    let mut c = 0;
    for i in 0..8 {
        for j in 0..8 {
            if (x + i + y + j) % 2 == 0 {
                c += if b[x + i][y + j] == 'B' { 1 } else { 0 }
            } else if b[x + i][y + j] == 'W' {
                c += 1;
            }
        }
    }
    c.min(64 - c)
}