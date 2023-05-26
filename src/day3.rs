use std::collections::HashSet;


pub fn common_items_priorities_sum(input_str: &str) -> u32 {
    let result = input_str
                                .split("\n")
                                .map(|s| s.trim()) 
                                .filter_map(|rucksack| {
                                    let mid = rucksack.len()/2;
                                    let (first_cmp, second_cmp) = rucksack.split_at(mid);
                                    let set: HashSet<_> = first_cmp.chars().collect();
                                    second_cmp.chars().find(|c| set.contains(c))
                                    .map(|c| {
                                        dbg!(rucksack, c);
                                        if c.is_lowercase() {
                                            ((c as u8) - ('a' as u8) + 1) as u32
                                        }
                                        else {
                                            ((c as u8) - ('A' as u8) + 27) as u32
                                        }
                                    }) 
                                }).sum();
    result
}

pub fn common_items_priorities_sum_part2(input_vec: Vec<String>) -> u32 {
    let result = input_vec
                                .chunks(3)
                                .map(|chunk| {
                                    let mut sets = chunk.iter().map(|l| {
                                        HashSet::from_iter(l.chars())
                                    }).collect::<Vec<HashSet<char>>>();
                                    let mut common_set: HashSet<char> = sets.pop().unwrap();
                                    for set in sets {
                                        common_set = set.intersection(&common_set).copied().collect()
                                    }
                                    let c = common_set.into_iter().next().unwrap();
                                    match c {
                                        'a'..='z'=> (c as u32).checked_sub('a' as u32).unwrap() + 1,
                                        'A'..='Z'=> (c as u32).checked_sub('A' as u32).unwrap() + 27,
                                        _ => panic!("bad input")
                                    }
                                }).sum();
    result
}

#[cfg(test)]
mod tests {

    use crate::aoclib::read_lines;
    use crate::aoclib::parse_file;

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day3_part1_exmaple_test() {
        assert_eq!(157u32, common_items_priorities_sum(INPUT))
    }

    #[test]
    fn day3_part1_input_test() {
        let input = parse_file("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day3.txt");
        assert_eq!(8139u32, common_items_priorities_sum(&input));
    }

    #[test]
    fn day3_part2_exmaple_test() {
        assert_eq!(70u32, common_items_priorities_sum_part2(INPUT.split('\n').map(|s| s.trim()).map(|s| s.to_string()).collect()))
    }

    #[test]
    fn day3_part2_input_test() {
        let input = read_lines("/Users/omrishneor/code/advent_of_code_2022/src/inputs/day3.txt");
        assert_eq!(2668u32, common_items_priorities_sum_part2(input));
    }    
}
