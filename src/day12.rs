use std::collections::HashMap;

type CaveId = usize;

struct Cave {
    name: String,
    neighbors: Vec<CaveId>,
    is_small: bool,
    visit_count: usize,
}

impl Cave {
    fn new(n: &str) -> Self {
        assert!(!n.is_empty());

        Cave {
            name: n.to_string(),
            neighbors: vec![],
            is_small: n.starts_with(|c: char| c.is_lowercase()),
            visit_count: 0,
        }
    }

    fn add_neighbor(&mut self, id: CaveId) {
        self.neighbors.push(id);
    }
}

struct Network {
    caves: Vec<Cave>,
    start: CaveId,
    end: CaveId,
    allows_two_visits: bool,
}

impl Network {
    fn from_edges<I, T>(edges: I) -> Self
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut nw = Network {
            caves: vec![],
            start: 0,
            end: 0,
            allows_two_visits: false,
        };
        let mut members = HashMap::new();

        for e in edges {
            let cnames: Vec<&str> = e.as_ref().trim().split('-').collect();
            if cnames.len() != 2 {
                continue;
            }
            let c0 = cnames[0];
            let c0_id = match members.get(c0) {
                Some(id) => *id,
                None => {
                    nw.caves.push(Cave::new(c0));
                    let id = nw.caves.len() - 1;
                    members.insert(c0.to_string(), id);
                    id
                }
            };
            let c1 = cnames[1];
            let c1_id = match members.get(c1) {
                Some(id) => *id,
                None => {
                    nw.caves.push(Cave::new(c1));
                    let id = nw.caves.len() - 1;
                    members.insert(c1.to_string(), id);
                    id
                }
            };
            nw.caves[c0_id].add_neighbor(c1_id);
            nw.caves[c1_id].add_neighbor(c0_id);
            if !nw.caves[c0_id].is_small && !nw.caves[c1_id].is_small {
                panic!("two adjacent large caves in input");
            }
        }

        nw.start = *members.get("start").expect("no start cave in input");
        nw.end = *members.get("end").expect("no end cave in input");
        nw
    }

    fn find_end(&mut self) -> usize {
        self.find_end_impl(self.start, true)
    }

    fn find_end_impl(&mut self, begin: CaveId, second_ok: bool) -> usize {
        if begin == self.end {
            return 1;
        }
        let mut paths = 0;
        self.caves[begin].visit_count += 1;

        let mut visit_next = vec![];
        for nir in &self.caves[begin].neighbors {
            let ni = *nir;
            if ni == self.start {
                continue;
            }
            if !self.caves[ni].is_small || self.caves[ni].visit_count == 0 {
                // unvisited cave
                visit_next.push((ni, second_ok));
            } else {
                // small visited cave
                if self.allows_two_visits && second_ok {
                    visit_next.push((ni, false));
                }
            }
        }
        for (ni, sok) in visit_next {
            paths += self.find_end_impl(ni, sok);
        }

        self.caves[begin].visit_count -= 1;
        paths
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input1() {
        let input = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
        let mut nw = Network::from_edges(input.lines());
        assert_eq!(10, nw.find_end());
        nw.allows_two_visits = true;
        assert_eq!(36, nw.find_end());
    }

    #[test]
    fn puzzle_input2() {
        let input = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
        let mut nw = Network::from_edges(input.lines());
        assert_eq!(19, nw.find_end());
        nw.allows_two_visits = true;
        assert_eq!(103, nw.find_end());
    }

    #[test]
    fn puzzle_input3() {
        let input = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
        let mut nw = Network::from_edges(input.lines());
        assert_eq!(226, nw.find_end());
        nw.allows_two_visits = true;
        assert_eq!(3509, nw.find_end());
    }
}

pub fn solve<I: Iterator<Item = String>>(lines: I, part: u8) {
    let mut nw = Network::from_edges(lines);
    match part {
        1 => println!("part1: {}", nw.find_end()),
        2 => {
            nw.allows_two_visits = true;
            println!("part2: {}", nw.find_end())
        }
        _ => unreachable!(),
    }
}
