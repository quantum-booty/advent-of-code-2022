use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Range = (u32, u32);
type Ranges = (Range, Range);

fn parse_range(input: &str) -> IResult<&str, Range> {
    separated_pair(complete::u32, tag("-"), complete::u32)(input)
}

fn parse_line(line: &str) -> IResult<&str, Ranges> {
    separated_pair(parse_range, tag(","), parse_range)(line)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Ranges>> {
    separated_list1(newline, parse_line)(input)
}

fn solution_a(input: &str) -> u32 {
    let (_, parsed) = parse_input(input).unwrap();
    parsed
        .iter()
        .filter(|(a, b)| (a.1 >= b.1 && a.0 <= b.0) || (b.1 >= a.1 && b.0 <= a.0))
        .count() as u32
}

fn solution_b(input: &str) -> u32 {
    let (_, parsed) = parse_input(input).unwrap();
    parsed
        .iter()
        .filter(|(a, b)| b.1 >= a.0 && a.1 >= b.0)
        .count() as u32
}

fn main() {
    let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let input = include_str!("input.txt");
    println!("{}", solution_a(test_input));
    println!("{}", solution_a(input));
    println!("{}", solution_b(test_input));
    println!("{}", solution_b(input));
}
