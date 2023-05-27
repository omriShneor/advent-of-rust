
use std::fs::File;
use std::io::Read;

pub fn parse_file(path: &str) -> String {
    let mut file = File::open(path)
        .expect("Failed to open the file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file");
    contents
}

pub fn read_lines(path: &str) -> Vec<String> {
    parse_file(path)
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub trait Runner<T> {
    fn parse(&mut self, path: &str);
    fn part1(&self) -> T;
    fn part2(&self) -> T;
}
