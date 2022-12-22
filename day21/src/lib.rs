use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, anychar, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use Monkey::*;
use Operation::*;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

impl Operation {
    fn calculate(&self, left: i64, right: i64) -> i64 {
        match self {
            Add => left + right,
            Sub => left - right,
            Div => left / right,
            Mul => left * right,
        }
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Job(Operation, &'a str, &'a str),
    Number(i64),
}

pub fn solution_a(input: &str) -> i64 {
    let (_, monkeys) = parse(input).unwrap();
    let monkeys = HashMap::<&str, Monkey>::from_iter(monkeys);
    calculate(&monkeys, "root")
}

fn calculate(monkeys: &HashMap<&str, Monkey>, monkey: &str) -> i64 {
    match &monkeys[monkey] {
        Job(operation, left, right) => {
            operation.calculate(calculate(monkeys, left), calculate(monkeys, right))
        }
        Number(number) => *number,
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, Monkey)>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Monkey)> {
    separated_pair(
        take(4usize),
        tag(": "),
        alt((map(complete::i64, Number), parse_job)),
    )(input)
}

fn parse_job(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            take(4usize),
            delimited(space1, anychar, space1),
            take(4usize),
        )),
        |(left, operation, right)| {
            let operation = match operation {
                '+' => Add,
                '-' => Sub,
                '/' => Div,
                '*' => Mul,
                _ => unreachable!(),
            };
            Job(operation, left, right)
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 152);
        println!("{}", solution_a(INPUT));
    }
}
