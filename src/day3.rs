mod part1 {

    pub struct PowerMeter<I, T>
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        ones_count: Vec<i32>,
        input: I,
    }

    impl<I, T> PowerMeter<I, T>
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        pub fn new(input: I, num_bits: usize) -> Self {
            PowerMeter {
                ones_count: vec![0; num_bits],
                input,
            }
        }

        fn process_next_input(&mut self, line: &str) {
            line.trim()
                .chars()
                .map(|c| match c {
                    '0' => -1,
                    '1' => 1,
                    _ => {
                        panic!("unexpected char: {}", c);
                    }
                })
                .zip(self.ones_count.iter_mut())
                .for_each(|(delta, curr)| {
                    *curr += delta;
                });
        }

        fn curr_value(&self) -> u64 {
            let gamma_rate: String = self
                .ones_count
                .iter()
                .map(|v| if *v > 0 { '1' } else { '0' })
                .collect();

            let epsilon_rate: String = self
                .ones_count
                .iter()
                .map(|v| if *v <= 0 { '1' } else { '0' })
                .collect();

            let gamma_rate = u64::from_str_radix(&gamma_rate, 2).unwrap();
            let epsilon_rate = u64::from_str_radix(&epsilon_rate, 2).unwrap();

            gamma_rate * epsilon_rate
        }
    }

    impl<I, T> Iterator for PowerMeter<I, T>
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            if let Some(line) = self.input.next() {
                self.process_next_input(line.as_ref());
                Some(self.curr_value())
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn puzzle_input() {
            let lines = [
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter();
            let meter = PowerMeter::new(lines, 5);
            assert_eq!(Some(198), meter.last());
        }

        #[test]
        fn two_bits() {
            let lines = ["00", "01", "11"].iter();
            let meter = PowerMeter::new(lines, 2);
            assert_eq!(Some(2), meter.last());
        }
    }
}

mod part2 {
    struct TrieNode {
        node_count: u32,
        zero: Option<Box<TrieNode>>,
        one: Option<Box<TrieNode>>,
    }

    impl TrieNode {
        fn new() -> Self {
            TrieNode {
                node_count: 0,
                zero: None,
                one: None,
            }
        }

        fn is_leaf(&self) -> bool {
            self.zero.is_none() && self.one.is_none()
        }
    }

    struct BitwiseTrie {
        root: Box<TrieNode>,
    }

    impl BitwiseTrie {
        pub fn new() -> Self {
            BitwiseTrie {
                root: Box::new(TrieNode::new()),
            }
        }

        pub fn insert(&mut self, line: &str) {
            let mut curr = &mut self.root;

            for c in line.trim().chars() {
                curr = match c {
                    '0' => curr.zero.get_or_insert_with(|| Box::new(TrieNode::new())),
                    '1' => curr.one.get_or_insert_with(|| Box::new(TrieNode::new())),
                    _ => {
                        panic!("unexpected input");
                    }
                };
                curr.node_count += 1;
            }
        }

        pub fn traverse<F>(&self, follow_one: F) -> String
        where
            F: Fn(u32, u32) -> bool,
        {
            let mut curr = Some(&self.root);
            let mut result = String::new();

            while let Some(c) = curr {
                if c.is_leaf() {
                    break;
                }
                let zeros = c.zero.as_ref().map_or(0, |n| n.node_count);
                let ones = c.one.as_ref().map_or(0, |n| n.node_count);

                if zeros == 0 {
                    result.push('1');
                    curr = c.one.as_ref();
                } else if ones == 0 {
                    result.push('0');
                    curr = c.zero.as_ref();
                } else if follow_one(ones, zeros) {
                    result.push('1');
                    curr = c.one.as_ref();
                } else {
                    result.push('0');
                    curr = c.zero.as_ref();
                }
            }
            result
        }
    }

    pub fn solve<I, T>(lines: I) -> u64
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut trie = BitwiseTrie::new();
        for line in lines {
            trie.insert(line.as_ref());
        }

        let o2_gen = trie.traverse(|ones, zeros| ones >= zeros);
        let o2_gen = u64::from_str_radix(&o2_gen, 2).unwrap();

        let co2_scrub = trie.traverse(|ones, zeros| ones < zeros);
        let co2_scrub = u64::from_str_radix(&co2_scrub, 2).unwrap();

        println!("o2: {}, co2: {}", o2_gen, co2_scrub);
        o2_gen * co2_scrub
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn simple_input() {
            let lines = ["111", "010"].iter();
            assert_eq!(14, solve(lines));
        }

        #[test]
        fn puzzle_input() {
            let lines = [
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter();
            assert_eq!(230, solve(lines));
        }
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => {
            let meter = part1::PowerMeter::new(lines, 5);
            println!("part1: {}", meter.last().unwrap());
        }
        2 => println!("part2: {}", part2::solve(lines)),
        _ => {}
    }
}
