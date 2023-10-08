fn main() {
    let mut v: Vec<u16> = (1..=9993).collect();
    for i in 1..=9973 {
        let (mut s, mut n) = (i, i);
        while n > 0 {
            s += n % 10;
            n /= 10;
        }
        v.retain(|&x| x != s);
    }
    v.iter().for_each(|n| {
        println!("{}", n);
    });
}