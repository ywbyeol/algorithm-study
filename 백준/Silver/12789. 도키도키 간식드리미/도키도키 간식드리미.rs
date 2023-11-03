fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().flat_map(str::parse);
    let (mut n, mut s) = (1, Vec::with_capacity(i.next().unwrap()));
    while let Some(c) = i.next() {
        if c == n {
            n += 1;
            while let Some(l) = s.last() {
                if l == &n {
                    s.pop();
                    n += 1;
                } else {
                    break;
                }
            }
        } else {
            s.push(c);
        }
    }
    let r = s.windows(2).all(|t| t[0] > t[1]);
    print!("{}", if r { "Nice" } else { "Sad" })
}