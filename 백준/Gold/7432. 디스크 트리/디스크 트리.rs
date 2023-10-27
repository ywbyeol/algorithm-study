use std::collections::BTreeMap;

fn main() {
    let mut t = T { r: N::n(None) };
    let l = std::io::stdin().lines().skip(1);
    l.for_each(|l| t.p(&l.unwrap()));
    t.r.s(0);
}

struct N {
    n: Option<String>,
    c: BTreeMap<String, N>,
}

impl N {
    fn n(n: Option<String>) -> Self {
        N {
            n,
            c: BTreeMap::new(),
        }
    }

    fn c(&mut self, n: &str) -> &mut N {
        self.c
            .entry(n.to_string())
            .or_insert(N::n(Some(n.to_string())))
    }

    fn s(&self, d: usize) {
        if let Some(ref n) = &self.n {
            println!("{:w$}{}", "", n, w = d - 1);
        }
        self.c.iter().for_each(|(_, c)| c.s(d + 1));
    }
}

struct T {
    r: N,
}

impl T {
    fn p(&mut self, p: &str) {
        let mut c = &mut self.r;
        for d in p.split('\\') {
            c = c.c(d);
        }
    }
}