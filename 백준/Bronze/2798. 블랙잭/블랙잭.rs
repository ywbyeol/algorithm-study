fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = s.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let (n, m, mut a) = (i.next().unwrap(), i.next().unwrap(), 0);
    let v: Vec<_> = i.collect();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let t = v[i] + v[j] + v[k];
                if t <= m && t > a {
                    a = t;
                }
            }
        }
    }
    println!("{}", a);
}