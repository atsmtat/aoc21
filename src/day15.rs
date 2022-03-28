const MAX_RISK: u32 = u32::max_value();

pub fn part1(grid: &[Vec<u32>]) -> u32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let row_size = grid[0].len();

    let mut total_risk = vec![MAX_RISK; row_size + 1];

    for row in grid {
        for tri in 1..total_risk.len() {
            let from_left = total_risk[tri - 1];
            let from_top = total_risk[tri];

            if from_left == MAX_RISK && from_top == MAX_RISK {
                total_risk[tri] = row[tri - 1];
            } else {
                total_risk[tri] = std::cmp::min(from_left, from_top) + row[tri - 1];
            }
        }
    }
    let mut result = *total_risk.last().unwrap();
    result -= grid[0][0];
    result
}

fn compute_lowest_risk(row: &[u32], total_risk: &mut [u32]) {
    for tri in 1..total_risk.len() {
        let from_left = total_risk[tri - 1];
        let from_top = total_risk[tri];

        if from_left == MAX_RISK && from_top == MAX_RISK {
            total_risk[tri] = row[tri - 1];
        } else {
            total_risk[tri] = std::cmp::min(from_left, from_top) + row[tri - 1];
        }
    }
}

pub fn part2(grid: &[Vec<u32>]) -> u32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    const REPEAT: usize = 5;
    let row_size = grid[0].len() * REPEAT;
    let mut total_risk = vec![MAX_RISK; row_size + 1];

    for bottom_rep_count in 0..REPEAT {
        for grid_row in grid {
            let mut extended_row = vec![];
            for right_rep_count in 0..REPEAT {
                extended_row.extend(grid_row.iter().map(|e| {
                    let ext_val = *e + bottom_rep_count as u32 + right_rep_count as u32;
                    if ext_val > 9 {
                        ext_val % 10 + 1
                    } else {
                        ext_val
                    }
                }));
            }
            println!(
                "{}",
                extended_row
                    .iter()
                    .map(|d| char::from_digit(*d, 10).unwrap())
                    .collect::<String>()
            );
            compute_lowest_risk(&extended_row, &mut total_risk);
        }
    }
    let mut result = *total_risk.last().unwrap();
    result -= grid[0][0];
    result
}

pub fn get_input_grid<I, T>(lines: I) -> Vec<Vec<u32>>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut grid = vec![];

    for line in lines.filter(|l| !l.as_ref().is_empty()) {
        grid.push(
            line.as_ref()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        )
    }
    grid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_example() {
        let input = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

        let grid = get_input_grid(input.lines());
        assert_eq!(40, part1(&grid));
        assert_eq!(315, part2(&grid));
    }

    #[test]
    fn part1_extended_9() {
        let input = "
91234
12345
23456
34567
45678
";

        let grid = get_input_grid(input.lines());
        assert_eq!(36, part1(&grid));
    }

    #[test]
    fn part2_9() {
        let input = "
9
";

        let grid = get_input_grid(input.lines());
        assert_eq!(36, part2(&grid));
    }

    #[test]
    fn part1_going_up() {
        let input = "
19999
19111
11191
";

        let grid = get_input_grid(input.lines());
        assert_eq!(8, part1(&grid)); // FAILS
    }
}

pub fn solve<I: Iterator<Item = String>>(lines: I, part: u8) {
    let grid = get_input_grid(lines);
    match part {
        1 => println!("{}", part1(&grid)),
        2 => println!("{}", part2(&grid)),
        _ => unreachable!(),
    }
}
