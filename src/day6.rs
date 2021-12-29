pub fn solve_impl<I, T>(mut lines: I, days: u32) -> usize
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut fishes: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .as_ref()
        .trim()
        .split(',')
        .map(|n| (1, n.parse::<u64>().unwrap()))
        .collect();

    for _ in 0..days {
        let mut new_fishes = 0;
        for (sibs, counter) in &mut fishes {
            if *counter == 0 {
                *counter = 6;
                new_fishes += *sibs;
            } else {
                *counter -= 1;
            }
        }
        fishes.push((new_fishes, 8));
    }

    fishes.iter().fold(0, |acc, (sibs, _)| acc + *sibs as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "3,4,3,1,2";
        assert_eq!(26984457539, solve_impl(input.lines(), 256));
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", solve_impl(lines, 80)),
        2 => println!("part2: {}", solve_impl(lines, 256)),
        _ => {}
    }
}
