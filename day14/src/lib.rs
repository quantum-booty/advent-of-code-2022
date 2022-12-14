use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
}

pub fn solution_a(input: &str) -> usize {
    let mut rocks = parse(input);
    let floor = rocks.iter().map(|coord| coord.y).max().unwrap();
    let mut number_sands = 0;
    loop {
        let mut sand = Coord { x: 500, y: 0 };
        while sand.y <= floor {
            // try move down
            if !rocks.contains(&Coord {
                x: sand.x,
                y: sand.y + 1,
            }) {
                sand.y += 1;
            }
            // try move left down
            else if !rocks.contains(&Coord {
                x: sand.x - 1,
                y: sand.y + 1,
            }) {
                sand.x -= 1;
                sand.y += 1;
            }
            // try move right down
            else if !rocks.contains(&Coord {
                x: sand.x + 1,
                y: sand.y + 1,
            }) {
                sand.x += 1;
                sand.y += 1;
            }
            // cannot move
            else {
                number_sands += 1;
                rocks.insert(sand);
                break;
            }
        }
        if sand.y > floor {
            break;
        }
    }
    number_sands
}

pub fn solution_b(input: &str) -> usize {
    let mut rocks = parse(input);
    let mut number_sands = 0;

    let floor = rocks.iter().map(|coord| coord.y).max().unwrap() + 2;
    loop {
        let mut sand = Coord { x: 500, y: 0 };
        loop {
            // try move down
            if !rocks.contains(&Coord {
                x: sand.x,
                y: sand.y + 1,
            }) {
                sand.y += 1;
            }
            // try move left down
            else if !rocks.contains(&Coord {
                x: sand.x - 1,
                y: sand.y + 1,
            }) {
                sand.x -= 1;
                sand.y += 1;
            }
            // try move right down
            else if !rocks.contains(&Coord {
                x: sand.x + 1,
                y: sand.y + 1,
            }) {
                sand.x += 1;
                sand.y += 1;
            }
            // cannot move
            else {
                number_sands += 1;
                rocks.insert(sand);
                break;
            }

            // hit the floor, cannot move
            if sand.y + 1 == floor {
                number_sands += 1;
                rocks.insert(sand);
                break;
            }
        }

        if sand.x == 500 && sand.y == 0 {
            break;
        }
    }
    number_sands
}

fn parse(input: &str) -> HashSet<Coord> {
    let (_, coords) = parse_coords(input).unwrap();
    coords
        .iter()
        .flat_map(|path| {
            path.iter().tuple_windows().flat_map(|(from, to)| {
                if from.x == to.x {
                    (from.y.min(to.y)..from.y.max(to.y) + 1)
                        .map(|y| Coord { x: from.x, y })
                        .collect::<Vec<Coord>>()
                } else {
                    (from.x.min(to.x)..from.x.max(to.x) + 1)
                        .map(|x| Coord { x, y: from.y })
                        .collect::<Vec<Coord>>()
                }
            })
        })
        .collect::<HashSet<Coord>>()
}

fn parse_coords(input: &str) -> IResult<&str, Vec<Vec<Coord>>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(
        tag(" -> "),
        map(
            separated_pair(complete::u32, tag(","), complete::u32),
            |(x, y)| Coord { x, y },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 24);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT), 93);
        println!("{}", solution_b(INPUT));
    }
}
