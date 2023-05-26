enum Move {
    Rock,
    Paper,
    Scissors,
}

enum MatchScore {
    Lost,
    Draw,
    Win
}

impl MatchScore {
    fn decrypt(input: &str) -> Option<Self> {
        match input {
            "X" => Some(Self::Lost),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None
        }
    }
}

impl Move {
    fn decrypt(input: &str) -> Option<Self> {
        match input {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}



pub fn rock_paper_scissors(input_str: &str) -> u32 {
    let result: u32 = input_str.lines()
                        .map(|line| {
                            let mut split_values = line.split_whitespace();
                            let rival = split_values.next().and_then(Move::decrypt).unwrap();
                            let me = split_values.next().and_then(Move::decrypt).unwrap();
                            (rival, me)
                        }).fold(0, |acc, (rival_move, my_move)| {
                            match (&my_move, &rival_move) {
                                (Move::Rock, Move::Paper) => acc+my_move.score(),
                                (Move::Rock, Move::Scissors) => acc+my_move.score()+6,
                                (Move::Paper, Move::Scissors) => acc+my_move.score(),
                                (Move::Paper, Move::Rock) => acc+my_move.score()+6,
                                (Move::Scissors, Move::Rock) => acc+my_move.score(),
                                (Move::Scissors, Move::Paper) => acc+my_move.score()+6,
                                _ => acc+ my_move.score() +3
                            }
                        });
                              
    result
}

pub fn rock_paper_scissors_part2(input_str: &str) -> u32 {
    let result: u32 = input_str.lines()
                        .map(|line| {
                            let mut split_values = line.split_whitespace();
                            let rival = split_values.next().and_then(Move::decrypt).unwrap();
                            let me = split_values.next().and_then(MatchScore::decrypt).unwrap();
                            (rival, me)
                        }).fold(0, |acc, (rival_move, my_score)| {
                            match (&rival_move, &my_score) {
                                (Move::Rock, MatchScore::Lost) => acc+Move::Scissors.score(),
                                (Move::Rock, MatchScore::Win) => acc+Move::Paper.score()+6,
                                (Move::Paper, MatchScore::Lost) => acc+Move::Rock.score(),
                                (Move::Paper, MatchScore::Win) => acc+Move::Scissors.score()+6,
                                (Move::Scissors, MatchScore::Lost) => acc+Move::Paper.score(),
                                (Move::Scissors, MatchScore::Win) => acc+Move::Rock.score()+6,
                                _ => acc+ rival_move.score() +3
                            }
                        });
                              
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoclib::parse_file;

    const INPUT: &str = 
"A Y
B X
C Z";

    #[test]
    fn day2_part1_example_test() {
        assert_eq!(15u32, rock_paper_scissors(INPUT));
    }

    #[test]
    fn day2_part1_input_test() {
        let input = parse_file("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day2.txt");
        assert_eq!(10994u32, rock_paper_scissors(&input));
    }

    #[test]
    fn day2_part2_example_test() {
        assert_eq!(12u32, rock_paper_scissors_part2(INPUT));
    }

    #[test]
    fn day2_part2_input_test() {
        let input = parse_file("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day2.txt");
        assert_eq!(12526u32, rock_paper_scissors_part2(&input));
    }
}
