use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn fold_y(points: &HashSet<Point>, split_y: usize) -> HashSet<Point> {
    let mut points_seen = HashSet::new();
    for point in points {
        if point.y > split_y {
            if 2 * split_y >= point.y {
                points_seen.insert(Point {
                    x: point.x,
                    y: 2 * split_y - point.y,
                });
            }
        } else {
            points_seen.insert(point.clone());
        }
    }
    points_seen
}

fn fold_x(points: &HashSet<Point>, split_x: usize) -> HashSet<Point> {
    let mut points_seen = HashSet::new();
    for point in points {
        if point.x > split_x {
            if 2 * split_x >= point.x {
                points_seen.insert(Point {
                    x: 2 * split_x - point.x,
                    y: point.y,
                });
            }
        } else {
            points_seen.insert(point.clone());
        }
    }
    points_seen
}

pub fn solve_impl<I, T>(lines: I) -> usize
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut points = HashSet::new();

    for line in lines {
        if line.as_ref().contains(',') {
            let ps: Vec<&str> = line.as_ref().split(',').collect();
            points.insert(Point {
                x: ps[0].parse::<usize>().unwrap(),
                y: ps[1].parse::<usize>().unwrap(),
            });
        }

        if line.as_ref().contains('=') {
            let folds: Vec<&str> = line.as_ref().split('=').collect();
            let split = folds[1].parse::<usize>().unwrap();

            if folds[0].ends_with('y') {
                points = fold_y(&points, split);
            }

            if folds[0].ends_with('x') {
                points = fold_x(&points, split);
            }
        }
    }

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let mut plot = vec![vec!['.'; max_x + 1]; max_y + 1];

    for point in &points {
        plot[point.y][point.x] = '#';
    }

    for y_line in &plot {
        println!("{}", y_line.iter().collect::<String>());
    }
    points.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_example() {
        let input = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
";
        assert_eq!(17, solve_impl(input.lines()));
    }
}

pub fn solve<I: Iterator<Item = String>>(lines: I, _part: u8) {
    solve_impl(lines);
}
