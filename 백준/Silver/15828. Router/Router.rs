use std::collections::VecDeque;

fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().flat_map(str::parse);
    let (n, mut q) = (i.next().unwrap(), VecDeque::new());
    for p in i.take_while(|&p| p != -1) {
        if p == 0 {
            q.pop_front();
        } else if q.len() as i32 != n {
            q.push_back(p);
        }
    }
    if q.is_empty() {
        print!("empty");
    } else {
        q.iter().for_each(|p| print!("{} ", p));
    }
}