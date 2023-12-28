fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let (n, mut k) = (s.next().unwrap(), s.next().unwrap());
    let mut s = vec![true; n + 1];
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            if s[j] {
                s[j] = false;
                if k == 1 {
                    print!("{}", j);
                    return;
                }
                k -= 1;
            }
        }
    }
}