use std::collections::VecDeque;

#[derive(Debug)]
struct EnergySim {
    map: Vec<Vec<i32>>,
}

#[derive(Debug, Clone)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    pub fn new(row: usize, col: usize) -> Self {
        Location { row, col }
    }
}

const FLASHED: i32 = -1;

impl EnergySim {
    pub fn from_lines<I, T>(lines: I) -> Self
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut map = vec![];

        for line in lines.filter(|l| !l.as_ref().trim().is_empty()) {
            let row = line
                .as_ref()
                .chars()
                .map(|c| c.to_digit(10).expect("invalid character in map") as i32)
                .collect();
            map.push(row);
        }

        EnergySim { map }
    }

    fn row_len(&self) -> usize {
        self.map.len()
    }

    fn col_len(&self) -> usize {
        if let Some(r) = self.map.first() {
            r.len()
        } else {
            0
        }
    }

    fn get(&self, loc: &Location) -> i32 {
        self.map[loc.row][loc.col]
    }

    fn set(&mut self, loc: &Location, val: i32) {
        self.map[loc.row][loc.col] = val;
    }

    fn neighbours(&self, loc: &Location) -> Vec<(Location, i32)> {
        let mut result = vec![];

        for r in [
            loc.row.checked_sub(1),
            Some(loc.row),
            loc.row.checked_add(1),
        ] {
            for c in [
                loc.col.checked_sub(1),
                Some(loc.col),
                loc.col.checked_add(1),
            ] {
                match (r, c) {
                    (Some(nr), Some(nc)) if nr < self.row_len() && nc < self.col_len() => {
                        result.push((Location::new(nr, nc), self.map[nr][nc]));
                    }
                    (Some(nr), Some(nc)) if nr == loc.row && nc == loc.col => {}
                    _ => {}
                }
            }
        }
        result
    }

    pub fn take_step(&mut self) -> usize {
        for e in self.map.iter_mut().flatten() {
            *e += 1;
        }
        let mut count = 0;
        for ri in 0..self.row_len() {
            for ci in 0..self.col_len() {
                let loc = Location::new(ri, ci);
                let val = self.get(&loc);
                if val > 9 && val != FLASHED {
                    count += self.propagate(&loc);
                }
            }
        }

        for e in self.map.iter_mut().flatten() {
            if *e == FLASHED {
                *e = 0;
            }
        }
        count
    }

    fn propagate(&mut self, loc: &Location) -> usize {
        let mut bfs_que = VecDeque::from([loc.clone()]);
        self.set(loc, FLASHED);
        let mut count = 1;

        while let Some(curr_loc) = bfs_que.pop_front() {
            for (next_loc, next_val) in self.neighbours(&curr_loc) {
                if next_val != FLASHED {
                    self.set(&next_loc, self.get(&next_loc) + 1);
                    if self.get(&next_loc) > 9 {
                        self.set(&next_loc, FLASHED);
                        bfs_que.push_back(next_loc);
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn part1(es: &mut EnergySim) -> usize {
    (0..100).map(|_| es.take_step()).sum()
}

fn part2(es: &mut EnergySim) -> usize {
    let mut steps = 1;
    loop {
        if es.take_step() == es.row_len() * es.col_len() {
            break;
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
        let mut es1 = EnergySim::from_lines(input.lines());
        assert_eq!(1656, part1(&mut es1));

        let mut es2 = EnergySim::from_lines(input.lines());
        assert_eq!(195, part2(&mut es2));
    }
}

pub fn solve<I: Iterator<Item = String>>(lines: I, part: u8) {
    let mut es = EnergySim::from_lines(lines);
    match part {
        1 => println!("part1: {}", part1(&mut es)),
        2 => println!("part2: {}", part2(&mut es)),
        _ => {}
    }
}
