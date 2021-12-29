use std::cmp;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

struct Segment {
    pub point1: Point,
    pub point2: Point,
}

impl Segment {
    pub fn new(line: &str) -> Self {
        let points: Vec<Point> = line
            .trim()
            .split("->")
            .map(|p| {
                let ends: Vec<i32> = p
                    .trim()
                    .split(',')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect();
                Point {
                    x: ends[0],
                    y: ends[1],
                }
            })
            .collect();
        Segment {
            point1: points[0].clone(),
            point2: points[1].clone(),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.point1.y == self.point2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.point1.x == self.point2.x
    }

    pub fn iter(&self) -> SegmentIter<RangeInclusive<i32>> {
        let x1 = self.point1.x;
        let x2 = self.point2.x;
        let y1 = self.point1.y;
        let y2 = self.point2.y;

        if self.is_horizontal() {
            SegmentIter::Horizontal(cmp::min(x1, x2)..=cmp::max(x1, x2), self.point1.y)
        } else if self.is_vertical() {
            SegmentIter::Vertical(cmp::min(y1, y2)..=cmp::max(y1, y2), self.point1.x)
        } else {
            let x_diff = x1 - x2;
            let y_diff = y1 - y2;
            let x_range = cmp::min(x1, x2)..=cmp::max(x1, x2);
            let y_range = cmp::min(y1, y2)..=cmp::max(y1, y2);

            if x_diff / y_diff > 0 {
                SegmentIter::Diagonal(x_range, y_range)
            } else {
                SegmentIter::DiagonalNeg(x_range, y_range)
            }
        }
    }
}

enum SegmentIter<I>
where
    I: DoubleEndedIterator<Item = i32>,
{
    Horizontal(I, i32),
    Vertical(I, i32),
    Diagonal(I, I),
    DiagonalNeg(I, I),
}

impl<I> Iterator for SegmentIter<I>
where
    I: DoubleEndedIterator<Item = i32>,
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Horizontal(iter, y) => iter.next().map(|x| Point::new(x, *y)),
            Self::Vertical(iter, x) => iter.next().map(|y| Point::new(*x, y)),
            Self::Diagonal(x_iter, y_iter) => {
                x_iter.next().map(|x| Point::new(x, y_iter.next().unwrap()))
            }
            Self::DiagonalNeg(x_iter, y_iter) => x_iter
                .next()
                .map(|x| Point::new(x, y_iter.next_back().unwrap())),
        }
    }
}

pub fn solve_impl<I, T>(lines: I, no_diag: bool) -> usize
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let segments: Vec<Segment> = lines
        .filter(|l| !l.as_ref().is_empty())
        .map(|l| Segment::new(l.as_ref()))
        .filter(|s| !no_diag || s.is_horizontal() || s.is_vertical())
        .collect();

    let mut grid = HashMap::new();
    for seg in segments {
        for point in seg.iter() {
            let count = grid.entry(point).or_insert(0);
            *count += 1;
        }
    }

    grid.values().filter(|&v| *v >= 2).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
        assert_eq!(5, solve_impl(input.lines(), true));
        assert_eq!(12, solve_impl(input.lines(), false));
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", solve_impl(lines, true)),
        2 => println!("part2: {}", solve_impl(lines, false)),
        _ => {}
    }
}
