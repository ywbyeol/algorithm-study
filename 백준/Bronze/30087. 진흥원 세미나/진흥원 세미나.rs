fn main() {
    for l in std::io::stdin().lines().skip(1) {
        let a = match l.unwrap().split_at(2).0 {
            "Al" => "204",
            "Da" => "207",
            "Ar" => "302",
            "Cy" => "B101",
            "Ne" => "303",
            "St" => "501",
            "Te" => "105",
            _ => "",
        };
        println!("{}", a);
    }
}