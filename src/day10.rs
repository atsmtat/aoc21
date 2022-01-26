use std::collections::HashMap;

enum Line {
    Corrupt(char),
    Incomplete(String),
}

fn corrupted_or_incomplete(line: &str) -> Line {
    let mut stack = vec![];

    let closing_chars = HashMap::from([(')', '('), ('}', '{'), (']', '['), ('>', '<')]);

    for c in line.chars() {
        if let Some(exp_opn) = closing_chars.get(&c) {
            if Some(exp_opn) == stack.last() {
                stack.pop();
            } else {
                return Line::Corrupt(c);
            }
        } else {
            stack.push(c);
        }
    }

    let remaining: String = stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("invalid char {}", c),
        })
        .collect();
    Line::Incomplete(remaining)
}

fn part1<I, T>(lines: I) -> u32
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    lines
        .filter_map(|l| match corrupted_or_incomplete(l.as_ref()) {
            Line::Corrupt(c) => Some(c),
            Line::Incomplete(_) => None,
        })
        .map(|c| match c {
            ')' => 3,
            '}' => 1197,
            ']' => 57,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn completion_score(txt: &str) -> u64 {
    txt.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("invalid char {}", c),
            }
    })
}

fn part2<I, T>(lines: I) -> u64
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut scores: Vec<u64> = lines
        .filter_map(|l| match corrupted_or_incomplete(l.as_ref()) {
            Line::Corrupt(_) => None,
            Line::Incomplete(compl) => Some(completion_score(&compl)),
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(26397, part1(input.lines()));
        assert_eq!(288957, part2(input.lines()));
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", part1(lines)),
        2 => println!("part2: {}", part2(lines)),
        _ => {}
    }
}
