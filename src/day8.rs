use std::collections::HashSet;

fn part1(entry: &str) -> u32 {
    if entry.trim().is_empty() {
        return 0;
    }
    let in_and_out: Vec<&str> = entry.split('|').collect();
    let digits: Vec<&str> = in_and_out[1].trim().split(' ').collect();

    digits
        .iter()
        .map(|d| match d.len() {
            2 | 4 | 3 | 7 => 1,
            _ => 0,
        })
        .sum()
}

fn sort_pattern(p: &str) -> String {
    let mut chars: Vec<char> = p.chars().collect();
    chars.sort_unstable();
    chars.iter().collect()
}

fn part2(entry: &str) -> u32 {
    if entry.trim().is_empty() {
        return 0;
    }

    let in_and_out: Vec<&str> = entry.split('|').collect();
    let patterns: Vec<&str> = in_and_out[0].trim().split(' ').collect();
    let digits: Vec<&str> = in_and_out[1].trim().split(' ').collect();

    let mut wiring = vec![String::new(); 10];
    let mut five_len_pat = HashSet::new();
    let mut six_len_pat = HashSet::new();

    for p in &patterns {
        match p.len() {
            2 => {
                wiring[1] = sort_pattern(p);
            }
            4 => {
                wiring[4] = sort_pattern(p);
            }
            3 => {
                wiring[7] = sort_pattern(p);
            }
            7 => {
                wiring[8] = sort_pattern(p);
            }
            5 => {
                five_len_pat.insert(sort_pattern(p));
            }
            6 => {
                six_len_pat.insert(sort_pattern(p));
            }
            _ => {}
        }
    }

    // five length pattern which contains all chars of 1 must be 3
    wiring[3] = five_len_pat
        .iter()
        .find(|p| wiring[1].chars().all(|c| p.contains(c)))
        .expect("pattern for 3 is missing")
        .to_string();
    five_len_pat.remove(&wiring[3]);

    // six length pattern which doesn't contain all chars of 1 must be 6
    wiring[6] = six_len_pat
        .iter()
        .find(|p| !wiring[1].chars().all(|c| p.contains(c)))
        .expect("pattern for 6 is missing")
        .to_string();
    six_len_pat.remove(&wiring[6]);

    // segment in 4's pattern which is not in 3's pattern must be b
    let sig_b = wiring[4]
        .chars()
        .find(|c| !wiring[3].contains(*c))
        .expect("failed to find b's mapping");

    wiring[5] = five_len_pat
        .iter()
        .find(|p| p.contains(sig_b))
        .expect("pattern for 5 is missing")
        .to_string();

    wiring[2] = five_len_pat
        .iter()
        .find(|p| !p.contains(sig_b))
        .expect("pattern for 2 is missing")
        .to_string();

    // segment in 6's pattern which is not in 5's pattern must be e
    let sig_e = wiring[6]
        .chars()
        .find(|c| !wiring[5].contains(*c))
        .expect("failed to find e's mapping");

    wiring[0] = six_len_pat
        .iter()
        .find(|p| p.contains(sig_e))
        .expect("pattern for 0 is missing")
        .to_string();

    wiring[9] = six_len_pat
        .iter()
        .find(|p| !p.contains(sig_e))
        .expect("pattern for 9 is missing")
        .to_string();

    let digits: Vec<usize> = digits
        .iter()
        .map(|d| {
            let (i, _) = wiring
                .iter()
                .enumerate()
                .find(|(_, p)| **p == sort_pattern(d))
                .unwrap();
            i
        })
        .collect();

    digits.iter().fold(0, |acc, d| 10 * acc + *d as u32)
}

fn solve_impl<I, T>(lines: I, part: u8) -> u32
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    match part {
        1 => lines.map(|l| part1(l.as_ref())).sum(),
        2 => lines.map(|l| part2(l.as_ref())).sum(),
        _ => {
            panic!("invalid part {}", part);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(26, solve_impl(input.lines(), 1));
        assert_eq!(61229, solve_impl(input.lines(), 2));
    }

    #[test]
    fn single_reading() {
        let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        assert_eq!(8394, solve_impl(input.lines(), 2));
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", solve_impl(lines, 1)),
        2 => println!("part2: {}", solve_impl(lines, 2)),
        _ => {}
    }
}
