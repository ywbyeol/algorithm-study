fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut n: u32 = n.trim().parse().unwrap();
    let mut l = 1;
    while n > l {
        n -= l;
        l += 1;
    }
    let (a, b) = if l % 2 == 0 {
        (n, l - n + 1)
    } else {
        (l - n + 1, n)
    };
    println!("{}/{}", a, b);
}