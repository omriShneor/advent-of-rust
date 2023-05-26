use crate::aoclib::{*, self};

struct Pair {
    pub bottom: u32,
    pub top: u32
}

impl Pair {
    pub fn new(s: String) -> Self {
        let mut split = s.split("-").into_iter();
        Pair {
            bottom: split.next().unwrap().parse::<u32>().unwrap(),
            top: split.next().unwrap().parse::<u32>().unwrap(),
        }
    }

    pub fn is_contained(&self, p: &Pair) -> bool {
        // Check if p is contained in self or not.
        self.bottom <= p.bottom && self.top >= p.top
    }

    pub fn is_overlapped(&self, p: &Pair) -> bool {
        // Check if p is overlapping with self or not.
        self.top >= p.bottom
    }
}

struct Aoc2022Day4 {
    lines: Vec<String>
}

impl Runner for Aoc2022Day4 {
    fn parse(&mut self, path: &str) {
        self.lines = aoclib::read_lines(path)
    }

    fn part1(&self) -> u32{
        self.lines.clone().into_iter().filter(|line| {
            let mut split = line.split(",").into_iter();
            let first_pair = Pair::new(split.next().unwrap().to_string());
            let second_pair = Pair::new(split.next().unwrap().to_string());
            first_pair.is_contained(&second_pair) || second_pair.is_contained(&first_pair)
        }).count() as u32
    }

    fn part2(&self) -> u32{
        self.lines.clone().into_iter().filter(|line| {
            let mut split = line.split(",").into_iter();
            let first_pair = Pair::new(split.next().unwrap().to_string());
            let second_pair = Pair::new(split.next().unwrap().to_string());
            (first_pair.bottom <= second_pair.bottom && first_pair.is_overlapped(&second_pair)) ||
                (second_pair.bottom <= first_pair.bottom && second_pair.is_overlapped(&first_pair))
        }).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";
    
    #[test]
    fn day4_part1_example() {
        let aoc = Aoc2022Day4 {lines: EXAMPLE.lines().map(|s| s.trim().to_string()).collect::<Vec<String>>()};
        assert_eq!(2u32, aoc.part1())
    }

    #[test]
    fn day4_part1_input() {
        let aoc = Aoc2022Day4 {lines: read_lines("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day4.txt")};
        assert_eq!(542u32, aoc.part1())
    }

    #[test]
    fn day4_part2_example() {
        let aoc = Aoc2022Day4 {lines: EXAMPLE.lines().map(|s| s.trim().to_string()).collect::<Vec<String>>()};
        assert_eq!(4u32, aoc.part2())
    }

    #[test]
    fn day4_part2_input() {
        let aoc = Aoc2022Day4 {lines: read_lines("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day4.txt")};
        assert_eq!(900u32, aoc.part2())
    }
}