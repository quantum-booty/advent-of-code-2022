use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let test_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let input = include_str!("input.txt");

    println!("{}", solution_a(test_input));
    println!("{}", solution_a(input));
    println!("{}", solution_b(test_input));
    println!("{}", solution_b(input));
}

fn solution_a(input: &str) -> String {
    let (mut stacks, cmds) = parse(input);
    for cmd in cmds {
        for _ in 0..cmd.quantity {
            if let Some(stuff) = stacks[cmd.from].pop() {
                stacks[cmd.to].push(stuff);
            }
        }
    }
    String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()))
}

fn solution_b(input: &str) -> String {
    let (mut stacks, cmds) = parse(input);
    for cmd in cmds {
        let len = stacks[cmd.from].len();
        for stuff in stacks[cmd.from]
            .drain(len - cmd.quantity..len)
            .collect::<Vec<char>>()
        {
            stacks[cmd.to].push(stuff);
        }
    }
    String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()))
}

#[derive(Debug)]
struct Cmd {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Cmd>) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(a);
    let (_, cmds) = parse_cmds(b).unwrap();
    (stacks, cmds)
}

fn parse_stacks(a: &str) -> Vec<Vec<char>> {
    let rev: Vec<&str> = a.lines().rev().collect();
    let positions: Vec<usize> = rev
        .first()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| c.is_numeric().then_some(i))
                .collect()
        })
        .unwrap();
    let stacks = positions
        .iter()
        .map(|n| {
            rev.iter()
                .skip(1)
                .map(|line| line.chars().nth(*n).unwrap())
                .filter(|c| c.is_ascii_alphabetic())
                .collect()
        })
        .collect();
    stacks
}

fn parse_cmds(input: &str) -> IResult<&str, Vec<Cmd>> {
    separated_list1(newline, parse_cmd)(input)
}

fn parse_cmd(line: &str) -> IResult<&str, Cmd> {
    let (remainder, (_, quantity, _, from, _, to)) = parse_line(line)?;
    Ok((
        remainder,
        Cmd {
            from: from as usize - 1,
            to: to as usize - 1,
            quantity: quantity as usize,
        },
    ))
}

fn parse_line(line: &str) -> IResult<&str, (&str, u32, &str, u32, &str, u32)> {
    tuple((
        tag("move "),
        complete::u32,
        tag(" from "),
        complete::u32,
        tag(" to "),
        complete::u32,
    ))(line)
}

// fn parse_cmds(input: &str) -> Vec<Cmd> {
//     input
//         .lines()
//         .map(|line| {
//             line.split_whitespace()
//                 .filter_map(|s| s.parse::<usize>().ok())
//                 .collect::<Vec<usize>>()
//         })
//         .map(|x| Cmd {
//             quantity: x[0],
//             from: x[1],
//             to: x[2],
//         })
//         .collect()
// }
