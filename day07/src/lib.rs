use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, not_line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};

#[derive(Debug)]
enum Entry {
    Cd(String),
    Ls,
    Dir(String),
    File(u32),
}

pub fn solution_a(input: &str) -> u32 {
    let (_, entries) = parse(input).unwrap();
    let sizes = calculate_dir_sizes(entries);
    sizes.values().filter(|size| **size <= 100000).sum()
}

pub fn solution_b(input: &str) -> u32 {
    let (_, entries) = parse(input).unwrap();
    let sizes = calculate_dir_sizes(entries);
    let unused = 70000000 - sizes["/"];
    let size_to_delete = 30000000 - unused;

    let mut sizes: Vec<u32> = sizes.values().cloned().collect();
    sizes.sort();
    *sizes.iter().find(|size| **size >= size_to_delete).unwrap()
}

fn calculate_dir_sizes(entries: Vec<Entry>) -> HashMap<String, u32> {
    let mut current_dir = vec![];
    let mut sizes = HashMap::<String, u32>::new();
    for e in entries {
        match e {
            Entry::Cd(dir) => match dir.as_str() {
                "/" => {
                    current_dir.clear();
                    current_dir.push(dir);
                }
                ".." => {
                    current_dir.pop();
                }
                _ => {
                    current_dir.push(dir);
                }
            },
            Entry::Ls => {}
            Entry::Dir(_) => {}
            Entry::File(size) => {
                for i in 0..current_dir.len() {
                    sizes
                        .entry(current_dir[..i + 1].join(","))
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
            }
        };
    }
    sizes
}

fn parse(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(line: &str) -> IResult<&str, Entry> {
    alt((
        map(
            pair(tag("$ cd "), not_line_ending::<&str, _>),
            |(_, dir)| Entry::Cd(dir.to_string()),
        ),
        map(tag("$ ls"), |_| Entry::Ls),
        map(pair(tag("dir "), not_line_ending::<&str, _>), |(_, dir)| {
            Entry::Dir(dir.to_string())
        }),
        map(
            pair(complete::u32, not_line_ending::<&str, _>),
            |(size, _)| Entry::File(size),
        ),
    ))(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 95437);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT), 24933642);
        println!("{}", solution_b(INPUT));
    }
}
