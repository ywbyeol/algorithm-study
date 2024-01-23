fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s = s.split_whitespace().skip(1).flat_map(str::parse::<usize>);
    let mut m = [[0; 100]; 100];
    for xy in Vec::from_iter(s).chunks(2) {
        (xy[1]..xy[1] + 10).for_each(|i| (xy[0]..xy[0] + 10).for_each(|j| m[i][j] = 1));
    }
    print!("{}", m.map(|v| v.iter().sum()).iter().sum::<u32>());
}