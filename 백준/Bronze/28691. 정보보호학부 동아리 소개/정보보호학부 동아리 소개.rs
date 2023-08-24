fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let a = match s.trim() {
        "M" => "MatKor",
        "W" => "WiCys",
        "C" => "CyKor",
        "A" => "AlKor",
        "$" => "$clear",
        _ => "",
    };
    println!("{}", a);
}