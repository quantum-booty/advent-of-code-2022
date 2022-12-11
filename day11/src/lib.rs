use std::collections::{BinaryHeap, HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, not_line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

enum Operation {
    Times,
    Add,
}

enum Value {
    Old,
    Number(u64),
}

struct Test {
    divisible: u64,
    case_true: usize,
    case_false: usize,
}

struct Monkey {
    items: Vec<u64>,
    operation: (Operation, Value),
    test: Test,
}

impl Monkey {
    fn operation(&self, worry: u64) -> u64 {
        let value = match self.operation.1 {
            Value::Old => worry,
            Value::Number(n) => n,
        };
        match self.operation.0 {
            Operation::Times => worry * value,
            Operation::Add => worry + value,
        }
    }

    fn test(&self, worry: u64) -> usize {
        if worry % self.test.divisible == 0 {
            self.test.case_true
        } else {
            self.test.case_false
        }
    }
}

fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        pair(space1, tag("Starting items: ")),
        separated_list1(tag(", "), complete::u64),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, (Operation, Value)> {
    preceded(
        pair(space1, tag("Operation: new = old ")),
        pair(
            alt((
                map(tag("* "), |_| Operation::Times),
                map(tag("+ "), |_| Operation::Add),
            )),
            alt((
                map(complete::u64, Value::Number),
                map(tag("old"), |_| Value::Old),
            )),
        ),
    )(input)
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let parse = tuple((
        preceded(pair(space1, tag("Test: divisible by ")), complete::u64),
        newline,
        preceded(
            pair(space1, tag("If true: throw to monkey ")),
            complete::u64,
        ),
        newline,
        preceded(
            pair(space1, tag("If false: throw to monkey ")),
            complete::u64,
        ),
    ));
    map(parse, |(divisible, _, case_true, _, case_false)| Test {
        divisible,
        case_true: case_true as usize,
        case_false: case_false as usize,
    })(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        preceded(
            pair(not_line_ending, newline),
            tuple((parse_items, newline, parse_operation, newline, parse_test)),
        ),
        |(items, _, operation, _, test)| Monkey {
            items,
            operation,
            test,
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(pair(newline, newline), parse_monkey)(input)
}

pub fn solution_a(input: &str) -> u64 {
    let (_, mut monkeys) = parse(input).unwrap();
    let mut inspections = HashMap::new();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            let worry_to: Vec<(u64, usize)> = monkey
                .items
                .iter()
                .map(|worry| {
                    let inspected_worry = monkey.operation(*worry);
                    let bored_worry = inspected_worry / 3;
                    let throw_to = monkey.test(bored_worry);
                    (bored_worry, throw_to)
                })
                .collect();

            inspections
                .entry(i)
                .and_modify(|n| *n += monkey.items.len() as u64)
                .or_insert(monkey.items.len() as u64);

            monkey.items.clear();

            for (worry, to) in worry_to {
                monkeys[to].items.push(worry);
            }
        }
    }

    let mut heap = BinaryHeap::from_iter(inspections.values());
    heap.pop().unwrap() * heap.pop().unwrap()
}

pub fn solution(input: &str, rounds: u32, part_a: bool) -> u64 {
    let (_, mut monkeys) = parse(input).unwrap();
    let mut inspections = HashMap::new();
    let ceil: u64 = monkeys.iter().map(|m| m.test.divisible).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            let worry_to: Vec<(u64, usize)> = monkey
                .items
                .iter()
                .map(|worry| {
                    let mut worry = monkey.operation(*worry);
                    if part_a {
                        worry /= 3;
                    } else {
                        worry %= ceil;
                    }
                    let throw_to = monkey.test(worry);
                    (worry, throw_to)
                })
                .collect();

            inspections
                .entry(i)
                .and_modify(|n| *n += monkey.items.len() as u64)
                .or_insert(monkey.items.len() as u64);

            monkey.items.clear();

            for (worry, to) in worry_to {
                monkeys[to].items.push(worry);
            }
        }
    }

    let mut heap = BinaryHeap::from_iter(inspections.values());
    heap.pop().unwrap() * heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solution(TEST_INPUT, 20, true), 10605);
        println!("{}", solution(INPUT, 20, true));

        assert_eq!(solution(TEST_INPUT, 10000, false), 2713310158);
        println!("{}", solution(INPUT, 10000, false));
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
