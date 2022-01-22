mod part1 {
    pub fn solve<I, T>(mut lines: I) -> u32
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut positions: Vec<u32> = lines
            .next()
            .unwrap()
            .as_ref()
            .trim()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        // sort positions
        positions.sort_unstable();

        let mut cost = 0;
        let mut crabs_to_move = 0;

        // compute costs to move crabs left of position i to i.
        let mut left_move_cost = vec![0; positions.len()];
        for i in 0..positions.len() {
            let steps = if i > 0 {
                positions[i] - positions[i - 1]
            } else {
                0
            };
            cost += crabs_to_move * steps;
            left_move_cost[i] = cost;
            crabs_to_move += 1;
        }

        // compute costs to move crabs right of position i to i.
        let mut right_move_cost = vec![0; positions.len()];
        crabs_to_move = 0;
        cost = 0;
        for i in (0..positions.len()).rev() {
            let steps = if i < positions.len() - 1 {
                positions[i + 1] - positions[i]
            } else {
                0
            };
            cost += crabs_to_move * steps;
            right_move_cost[i] = cost;
            crabs_to_move += 1;
        }

        let mut result = u32::max_value();
        for i in 0..positions.len() {
            result = std::cmp::min(result, left_move_cost[i] + right_move_cost[i]);
        }

        result
    }
}

mod part2 {
    pub fn solve<I, T>(mut lines: I) -> i32
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let positions: Vec<i32> = lines
            .next()
            .unwrap()
            .as_ref()
            .trim()
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let min = *positions.iter().min().unwrap_or(&0);
        let max = *positions.iter().max().unwrap_or(&0);

        (min..=max)
            .map(|i| {
                positions
                    .iter()
                    .map(|p| {
                        let d = (p - i).abs();
                        d * (d + 1) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(37, part1::solve(input.lines()));
        assert_eq!(168, part2::solve(input.lines()));
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", part1::solve(lines)),
        2 => println!("part2: {}", part2::solve(lines)),
        _ => {}
    }
}
