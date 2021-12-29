struct BingoBoard<const N: usize> {
    board: Vec<Vec<(u32, bool)>>,
    row_marked: Vec<u32>,
    col_marked: Vec<u32>,
    has_won: bool,
}

impl<const N: usize> BingoBoard<N> {
    pub fn from_lines(lines: &[String]) -> Self {
        let mut board = Vec::new();

        for line in lines {
            let row: Vec<(u32, bool)> = line
                .trim()
                .split_whitespace()
                .map(|num| (num.trim().parse::<u32>().unwrap(), false))
                .collect();
            assert!(row.len() == N);
            board.push(row);
        }
        assert!(board.len() == N);
        BingoBoard {
            board,
            row_marked: vec![0; N],
            col_marked: vec![0; N],
            has_won: false,
        }
    }

    pub fn mark_num(&mut self, num: u32) -> bool {
        for (ri, row) in self.board.iter_mut().enumerate() {
            for (ci, board_val) in row.iter_mut().enumerate() {
                if board_val.0 == num {
                    board_val.1 = true;
                    self.row_marked[ri] += 1;
                    self.col_marked[ci] += 1;
                }
            }
        }
        let row_complete = self.row_marked.iter().any(|&n| n as usize == N);
        let col_complete = self.col_marked.iter().any(|&n| n as usize == N);

        self.has_won = row_complete || col_complete;
        self.has_won
    }

    pub fn has_won(&self) -> bool {
        self.has_won
    }

    pub fn get_score(&self, last_num: u32) -> u32 {
        let mut unmarked_sum = 0;
        for row in &self.board {
            for entry in row {
                unmarked_sum += if entry.1 { 0 } else { entry.0 }
            }
        }
        unmarked_sum * last_num
    }
}

fn drawn_numbers(num_str: &str) -> Vec<u32> {
    num_str
        .trim()
        .split(',')
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect()
}

fn create_boards<I, T>(lines: I) -> Vec<BingoBoard<5>>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut boards: Vec<BingoBoard<5>> = Vec::new();

    let mut board_lines = Vec::new();
    for l in lines {
        board_lines.push(l.as_ref().to_string());
        if board_lines.len() == 5 {
            boards.push(BingoBoard::from_lines(&board_lines));
            board_lines.clear();
        }
    }
    boards
}

mod part1 {
    use super::*;
    pub fn solve<I, T>(lines: I) -> u32
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut lines = lines.filter(|l| !l.as_ref().is_empty());

        let seq = drawn_numbers(lines.next().unwrap().as_ref());

        let mut boards = create_boards(lines);

        for num in seq {
            if let Some(winner) =
                boards
                    .iter_mut()
                    .find_map(|b| if b.mark_num(num) { Some(b) } else { None })
            {
                return winner.get_score(num);
            }
        }

        // no winner
        0
    }
}

mod part2 {
    use super::*;
    pub fn solve<I, T>(lines: I) -> u32
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        let mut lines = lines.filter(|l| !l.as_ref().is_empty());

        let seq = drawn_numbers(lines.next().unwrap().as_ref());

        let mut boards = create_boards(lines);
        let mut last_winner = (0, 0);

        for num in seq {
            for (i, b) in boards.iter_mut().enumerate() {
                if !b.has_won() && b.mark_num(num) {
                    last_winner = (i, num);
                }
            }
        }
        boards[last_winner.0].get_score(last_winner.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_input() {
        let input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        assert_eq!(4512, part1::solve(input.lines()));
        assert_eq!(1924, part2::solve(input.lines()));
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
