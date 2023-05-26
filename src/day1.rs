use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn caolries_counting(input_str: &str) -> u32 {
    let result = input_str
                                .split("\n\n")
                                .map(|elf_carry| {
                                    elf_carry
                                        .lines()
                                        .map(|item| item.parse::<u32>().unwrap())
                                        .sum::<u32>()
                                }).max().unwrap();
    result
}

pub fn top3_calories_counting(input_str: &str) -> u32 {
    let mut heap= BinaryHeap::new();
    input_str.split("\n\n")
                .map(|elf_carry| {
                    elf_carry
                        .lines()
                        .map(|item| item.parse::<u32>().unwrap())
                        .sum::<u32>()
                }).for_each(|total_calories| {
                    heap.push(Reverse(total_calories));
                    if heap.len() > 3 {
                        heap.pop();
                    }
                });
    heap.into_iter().map(|reversed| reversed.0).sum()
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoclib::parse_file;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn day1_part1_example_test() {
        assert_eq!(24000u32, caolries_counting(INPUT));
    }
    #[test]
    fn day1_part1_input_test() {
        let input: String = parse_file("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day1.txt");
        assert_eq!(70698u32, caolries_counting(&input));
    }

    #[test]
    fn day1_part2_example_test() {
        assert_eq!(45000u32, top3_calories_counting(INPUT))
    }
}

