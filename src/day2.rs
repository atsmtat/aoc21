enum Cmd {
    Forward(u32),
    Up(u32),
    Down(u32),
    Invalid,
}

impl Cmd {
    pub fn new(line: &str) -> Self {
        let mut instr = line.trim().split_whitespace();
        let op = instr.next().unwrap_or("");
        let arg = instr.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
        match op {
            "forward" => Cmd::Forward(arg),
            "up" => Cmd::Up(arg),
            "down" => Cmd::Down(arg),
            _ => Cmd::Invalid,
        }
    }
}

mod part1 {
    use super::Cmd;

    pub fn execute_commands<I, T>(input: I) -> i64
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let final_pos =
            input
                .map(|s| Cmd::new(s.as_ref()))
                .fold((0i64, 0i64), |pos, cmd| match cmd {
                    Cmd::Forward(arg) => (pos.0 + arg as i64, pos.1),
                    Cmd::Up(arg) => (pos.0, pos.1 - arg as i64),
                    Cmd::Down(arg) => (pos.0, pos.1 + arg as i64),
                    Cmd::Invalid => pos,
                });

        final_pos.0 * final_pos.1
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn simple_seq() {
            let lines = ["forward 2", "down 4", "up 2"].iter();
            assert_eq!(4, execute_commands(lines));
        }

        #[test]
        fn flying_submarine() {
            let lines = ["forward 2", "down 4", "up 6"].iter();
            assert_eq!(-4, execute_commands(lines));
        }

        #[test]
        fn puzzle_example() {
            let lines = [
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .iter();
            assert_eq!(150, execute_commands(lines));
        }
    }
}

mod part2 {
    use super::Cmd;

    struct Submarine {
        position: (u32, u32),
        aim: u32,
    }

    pub fn execute_commands<I, T>(input: I) -> u32
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut submarine = Submarine {
            position: (0, 0),
            aim: 0,
        };
        let cmds = input.map(|l| Cmd::new(l.as_ref()));

        for cmd in cmds {
            match cmd {
                Cmd::Forward(arg) => {
                    submarine.position.0 += arg;
                    submarine.position.1 += submarine.aim * arg;
                }
                Cmd::Up(arg) => {
                    submarine.aim -= arg;
                }
                Cmd::Down(arg) => {
                    submarine.aim += arg;
                }
                Cmd::Invalid => {}
            }
        }

        submarine.position.0 * submarine.position.1
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn zero_depth() {
            let lines = ["forward 2", "down 4", "up 2"].iter();
            assert_eq!(0, execute_commands(lines));
        }

        #[test]
        fn couple_of_forwards() {
            let lines = ["down 2", "forward 100", "up 1", "forward 100"].iter();
            assert_eq!(60000, execute_commands(lines));
        }

        #[test]
        fn puzzle_example() {
            let lines = [
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .iter();
            assert_eq!(900, execute_commands(lines));
        }
    }
}

pub fn solve<I>(lines: I, part: u8)
where
    I: Iterator<Item = String>,
{
    match part {
        1 => println!("part1: {}", part1::execute_commands(lines)),
        2 => println!("part2: {}", part2::execute_commands(lines)),
        _ => {}
    }
}
