use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

use Instruction::*;

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let parse_line = alt((
        map(preceded(tag("addx "), complete::i32), Addx),
        map(tag("noop"), |_| Noop),
    ));
    separated_list1(newline, parse_line)(input)
}

pub fn solution(input: &str) -> i32 {
    let (_, instructions) = parse(input).unwrap();
    let mut x = 1;
    let mut cycle = 0;
    let mut signal = 0;
    let mut grid = vec![' '; 40 * 6];
    for inst in instructions {
        grid[cycle] = draw_pixel(cycle, x);
        cycle += 1;
        if cycle % 40 == 20 {
            signal += cycle as i32 * x;
        }

        if let Addx(v) = inst {
            grid[cycle] = draw_pixel(cycle, x);
            cycle += 1;
            if cycle % 40 == 20 {
                signal += cycle as i32 * x;
            }
            x += v;
        }
    }

    let image = grid
        .chunks(40)
        .map(|row| String::from_iter(row.iter()))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", image);

    signal
}

#[inline]
fn draw_pixel(cycle: usize, x: i32) -> char {
    if (cycle as i32 % 40 - x).abs() < 2 {
        '#'
    } else {
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solution(TEST_INPUT), 13140);
        println!("{}", solution(INPUT));
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
