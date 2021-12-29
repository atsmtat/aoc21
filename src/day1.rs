pub fn num_of_increments<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    let mut iter = input.map(|s| s.parse::<u64>().unwrap()).peekable();
    let mut sum = 0;

    while let Some(v) = iter.next() {
        if let Some(n) = iter.peek() {
            if n > &v {
                sum += 1;
            }
        }
    }
    sum
}

pub fn num_of_window_increments<I>(input: I, win_size: usize) -> usize
where
    I: Iterator<Item = String>,
{
    let lines: Vec<u64> = input.map(|s| s.parse::<u64>().unwrap()).collect();
    let mut increments = 0;

    let mut right = win_size;
    let mut left = 0;

    if right > lines.len() {
        return 0;
    }
    let mut prev_win: u64 = lines[left..right].iter().sum();

    while right < lines.len() {
        let next_win = prev_win - lines[left] + lines[right];
        if next_win > prev_win {
            increments += 1;
        }
        left += 1;
        right += 1;
        prev_win = next_win;
    }
    increments
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_window_increments() {
        let lines = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
            .iter()
            .map(|n| n.to_string());
        assert_eq!(num_of_window_increments(lines, 3), 5);
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", num_of_increments(lines)),
        2 => println!("part2: {}", num_of_window_increments(lines, 3)),
        _ => {}
    }
}
