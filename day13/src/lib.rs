use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use std::cmp::Ordering;
use Packet::*;

#[derive(Debug, PartialEq, Clone, Eq)]
enum Packet {
    Val(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Val(l), Val(r)) => l.partial_cmp(r),
            (Val(l), List(_)) => List(vec![Val(*l)]).partial_cmp(other),
            (List(_), Val(r)) => self.partial_cmp(&List(vec![Val(*r)])),
            (List(l), List(r)) => l.partial_cmp(r),
        }
    }
}

pub fn solution_a(input: &str) -> usize {
    let (_, pairs) = parse_a(input).unwrap();
    let mut answer = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let order = pair.0.partial_cmp(&pair.1);
        if order == Some(Ordering::Less) {
            answer += i + 1;
        }
    }
    answer
}

pub fn solution_b(input: &str) -> usize {
    let (_, mut packets) = parse_b(input.replace("\n\n", "\n").as_str()).unwrap();
    let (_, dividers) = parse_b(
        "[[2]]
[[6]]
",
    )
    .unwrap();
    packets.extend(dividers.clone());

    packets.sort();
    let first = packets.iter().position(|p| *p == dividers[0]).unwrap();
    let second = packets.iter().position(|p| *p == dividers[1]).unwrap();
    (first + 1) * (second + 1)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    delimited(
        tag("["),
        map(
            separated_list0(tag(","), alt((map(complete::u8, Val), parse_packet))),
            List,
        ),
        tag("]"),
    )(input)
}

fn parse_a(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        pair(newline, newline),
        separated_pair(parse_packet, newline, parse_packet),
    )(input)
}

fn parse_b(input: &str) -> IResult<&str, Vec<Packet>> {
    separated_list1(newline, parse_packet)(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 13);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT), 140);
        println!("{}", solution_b(INPUT));
    }
}
