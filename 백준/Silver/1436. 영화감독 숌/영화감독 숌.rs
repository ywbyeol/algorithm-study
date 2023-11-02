fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = (666..2666800).filter(|v| f(v));
    println!("{}", i.nth(n.trim().parse::<usize>().unwrap() - 1).unwrap());
}

fn f(n: &i32) -> bool {
    let (mut c, mut s) = (*n, 0);
    while c > 0 {
        if c % 10 == 6 {
            s += 1;
            if s == 3 {
                return true;
            }
        } else {
            s = 0;
        }
        c /= 10;
    }
    false
}