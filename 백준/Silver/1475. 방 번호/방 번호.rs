fn main() {
    let mut l = vec![0; 10];
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    for n in s.chars().flat_map(|c| c.to_digit(10)) {
        let t = if l[6] < l[9] { 6 } else { 9 };
        l[if n == 6 || n == 9 { t } else { n } as usize] += 1;
    }
    print!("{}", l.iter().max().unwrap());
}