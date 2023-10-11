fn main() {
    let i = std::io::stdin().lines().skip(1);
    i.map(|l| l.unwrap()).for_each(|v| {
        let mut v = v.split("").collect::<Vec<_>>();
        v.reverse();
        println!("{}", v.join(""));
    });
}