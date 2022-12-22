#![feature(int_roundings)]
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

#[derive(Debug, Clone, Copy)]
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

    fn inverse_calculate(&self, result: i64, left: Option<i64>, right: Option<i64>) -> i64 {
        // left = x Operation right
        match self {
            Add => result - left.or(right).unwrap(),
            Sub => match (left, right) {
                // result = x - right
                (None, Some(right)) => result + right,
                // result = left - x
                (Some(left), None) => left - result,
                _ => unreachable!(),
            },
            Div => match (left, right) {
                // result = x / right
                (None, Some(right)) => result * right,
                // result = right / x
                (Some(right), None) => right / result,
                _ => unreachable!(),
            },
            Mul => result / left.or(right).unwrap(),
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

#[derive(Debug)]
enum MonkeyRec {
    Job(Operation, Box<MonkeyRec>, Box<MonkeyRec>),
    Number(i64),
    Human,
}

fn calculate_rec(monkeys: &HashMap<&str, Monkey>, monkey: &str) -> MonkeyRec {
    if monkey == "humn" {
        return MonkeyRec::Human;
    }
    match &monkeys[monkey] {
        Job(operation, left, right) => {
            let left_result = calculate_rec(monkeys, left);
            let right_result = calculate_rec(monkeys, right);
            match (&left_result, &right_result) {
                (MonkeyRec::Number(left_result), MonkeyRec::Number(right_result)) => {
                    MonkeyRec::Number(operation.calculate(*left_result, *right_result))
                }
                _ => MonkeyRec::Job(*operation, Box::new(left_result), Box::new(right_result)),
            }
        }
        Number(number) => MonkeyRec::Number(*number),
    }
}

pub fn solution_b(input: &str) -> i64 {
    let (_, monkeys) = parse(input).unwrap();
    let monkeys = HashMap::<&str, Monkey>::from_iter(monkeys);

    match &monkeys["root"] {
        Job(_, left, right) => {
            let left = calculate_rec(&monkeys, left);
            let right = calculate_rec(&monkeys, right);
            match (&left, &right) {
                (MonkeyRec::Number(lhs), MonkeyRec::Job(..)) => solve_equality(*lhs, right),
                (MonkeyRec::Job(..), MonkeyRec::Number(lhs)) => solve_equality(*lhs, left),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn solve_equality(result: i64, rhs: MonkeyRec) -> i64 {
    match rhs {
        MonkeyRec::Job(operation, left, right) => {
            let left = *left;
            let right = *right;
            match (&left, &right) {
                (MonkeyRec::Number(left), MonkeyRec::Job(..)) => solve_equality(
                    operation.inverse_calculate(result, Some(*left), None),
                    right,
                ),
                (MonkeyRec::Job(..), MonkeyRec::Number(right)) => solve_equality(
                    operation.inverse_calculate(result, None, Some(*right)),
                    left,
                ),
                (MonkeyRec::Number(left), MonkeyRec::Human) => {
                    operation.inverse_calculate(result, Some(*left), None)
                }
                (MonkeyRec::Human, MonkeyRec::Number(right)) => {
                    operation.inverse_calculate(result, None, Some(*right))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
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
        assert_eq!(solution_b(TEST_INPUT), 301);
        println!("{}", solution_b(INPUT));
    }
}
