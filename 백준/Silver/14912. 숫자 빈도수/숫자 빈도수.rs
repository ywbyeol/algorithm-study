fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = s.split_whitespace();
    let mut f = || i.next().unwrap().parse::<i32>().unwrap();
    let (n, d, mut c) = (f(), f(), 0);
    for i in 1..=n {
        let mut n = i;
        while n > 0 {
            if n % 10 == d {
                c += 1;
            }
            n /= 10;
        }
    }
    println!("{}", c);
}