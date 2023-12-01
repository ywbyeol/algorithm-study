use std::collections::HashSet;

fn main() {
    let l = || std::io::stdin().lines().next().unwrap().unwrap();
    l();
    let p = || l().split_whitespace().flat_map(str::parse).collect();
    let a: HashSet<u32> = p();
    print!("{}", a.symmetric_difference(&p()).count());
}