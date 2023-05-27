use crate::aoclib::Runner;

struct Aoc2022Day5 {
    pub stacks: [Vec<char>; 3],
    pub instructions: Vec<(usize, usize, usize)>
}

impl Runner<String> for Aoc2022Day5 {
    fn parse(&mut self, path: &str) {
        todo!()
    }

    fn part1(&self) -> String {
        todo!()
    }

    fn part2(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    #[test]
    fn day5_part1_example() {
        let aoc = Aoc2022Day5 {stacks: [Vec::new(), Vec::new(), Vec::new()], instructions: Vec::new()};
        assert_eq!(aoc.part1(), "CMZ");
    }
}
