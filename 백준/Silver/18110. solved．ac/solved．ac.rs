fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let i = i.split_whitespace().skip(1).flat_map(str::parse);
    let mut l = Vec::from_iter(i);
    l.sort_unstable();
    let o = (l.len() as f32 * 0.15).round() as usize;
    let t = &l[o..l.len() - o];
    let t = t.iter().sum::<u32>() as f32 / t.len() as f32;
    println!("{}", t.round() as u32);
}