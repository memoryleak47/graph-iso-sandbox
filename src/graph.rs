type Node = usize;
type Neighbours = Vec<Node>;

#[derive(Debug)]
pub struct Graph(Vec<Neighbours>);

impl Graph {
    pub fn parse_file(filename: &str) -> Graph {
        use std::io::Read;
        use std::fs::File;

        let mut s = String::new();
        File::open(filename)
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        Self::parse(&s)
    }

    pub fn parse(s: &str) -> Graph {
        let mut edges = Vec::new();
        let mut max = 0;
        for line in s.split("\n") {
            let mut it = line.trim().split(" ");
            let Some("e") = it.next() else { continue };
            let Some(l) = it.next() else { continue };
            let Some(r) = it.next() else { continue };
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            edges.push((l, r));
            max = max.max(l).max(r);
        }

        let mut g = Vec::new();
        for i in 0..=max {
            g.push(Vec::new());
        }

        for (x, y) in edges {
            if !g[x].contains(&y) { g[x].push(y); }
            if !g[y].contains(&x) { g[y].push(x); }
        }

        Graph(g)
    }
}
