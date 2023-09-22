fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v = vec!["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];
    for c in v {
        s = s.replace(c, "1");
    }
    println!("{}", s.trim().len());
}