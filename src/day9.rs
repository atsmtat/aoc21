use std::collections::VecDeque;

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u32>>,
}

#[derive(Clone, Debug)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    pub fn new(row: usize, col: usize) -> Self {
        Location { row, col }
    }
}

impl HeightMap {
    pub fn from_lines<I, T>(lines: I) -> Self
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut map = vec![];
        for line in lines {
            let trim_line = line.as_ref().trim();
            if !trim_line.is_empty() {
                let row: Vec<u32> = trim_line
                    .chars()
                    .map(|c| c.to_digit(10).expect("map value must be between 0-9"))
                    .collect();
                map.push(row);
            }
        }
        HeightMap { map }
    }

    pub fn up(&self, loc: &Location) -> Option<(Location, u32)> {
        if loc.row == 0 {
            return None;
        }

        if let Some(row) = self.map.get(loc.row - 1) {
            if let Some(elem) = row.get(loc.col) {
                return Some((Location::new(loc.row - 1, loc.col), *elem));
            }
        }
        None
    }

    pub fn down(&self, loc: &Location) -> Option<(Location, u32)> {
        if let Some(row) = self.map.get(loc.row + 1) {
            if let Some(elem) = row.get(loc.col) {
                return Some((Location::new(loc.row + 1, loc.col), *elem));
            }
        }
        None
    }

    pub fn left(&self, loc: &Location) -> Option<(Location, u32)> {
        if loc.col == 0 {
            return None;
        }

        if let Some(row) = self.map.get(loc.row) {
            if let Some(elem) = row.get(loc.col - 1) {
                return Some((Location::new(loc.row, loc.col - 1), *elem));
            }
        }
        None
    }

    pub fn right(&self, loc: &Location) -> Option<(Location, u32)> {
        if let Some(row) = self.map.get(loc.row) {
            if let Some(elem) = row.get(loc.col + 1) {
                return Some((Location::new(loc.row, loc.col + 1), *elem));
            }
        }
        None
    }

    pub fn row_len(&self) -> usize {
        self.map.len()
    }

    pub fn col_len(&self) -> usize {
        if let Some(row) = self.map.get(0) {
            row.len()
        } else {
            0
        }
    }

    pub fn get(&self, loc: &Location) -> Option<u32> {
        if let Some(row) = self.map.get(loc.row) {
            if let Some(elem) = row.get(loc.col) {
                return Some(*elem);
            }
        }
        None
    }

    pub fn set(&mut self, loc: &Location, val: u32) {
        self.map[loc.row][loc.col] = val;
    }

    pub fn neighbours(&self, loc: &Location) -> Vec<(Location, u32)> {
        [
            self.up(loc),
            self.down(loc),
            self.left(loc),
            self.right(loc),
        ]
        .iter()
        .filter_map(|e| e.as_ref())
        .cloned()
        .collect()
    }
}

const VISITED: u32 = 10;
const PEAK: u32 = 9;

fn bfs(hm: &mut HeightMap, loc: &Location) -> usize {
    hm.set(loc, VISITED);
    let mut bfs_que = VecDeque::from([loc.clone()]);
    let mut count = 1;
    while let Some(curr_loc) = bfs_que.pop_front() {
        for next in hm.neighbours(&curr_loc) {
            let (next_loc, next_elem) = next;
            if next_elem != VISITED && next_elem != PEAK {
                hm.set(&next_loc, VISITED);
                bfs_que.push_back(next_loc);
                count += 1;
            }
        }
    }
    count
}

fn basins(hm: &mut HeightMap) -> Vec<usize> {
    let mut result = vec![];
    for r in 0..hm.row_len() {
        for c in 0..hm.col_len() {
            let loc = Location::new(r, c);
            match hm.get(&loc) {
                Some(e) if (e != VISITED && e != PEAK) => {
                    result.push(bfs(hm, &loc));
                }
                _ => {}
            }
        }
    }
    result.sort_unstable();
    result
}

fn part2(hm: &mut HeightMap) -> usize {
    basins(hm).iter().rev().take(3).product()
}

fn part1(hm: &HeightMap) -> u32 {
    let mut result = 0;
    for r in 0..hm.row_len() {
        result += (0..hm.col_len())
            .map(|c| {
                let curr = hm.map[r][c];
                if hm
                    .neighbours(&Location::new(r, c))
                    .iter()
                    .all(|n| curr < n.1)
                {
                    1 + curr
                } else {
                    0
                }
            })
            .sum::<u32>();
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "
2199943210
3987894921
9856789892
8767896789
9899965678
";
        let mut hm = HeightMap::from_lines(input.lines());
        assert_eq!(15, part1(&hm));
        assert_eq!(1134, part2(&mut hm));
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    let mut hm = HeightMap::from_lines(lines);
    match part {
        1 => println!("part1: {}", part1(&hm)),
        2 => println!("part2: {}", part2(&mut hm)),
        _ => {}
    }
}
