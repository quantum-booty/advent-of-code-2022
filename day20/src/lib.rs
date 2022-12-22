#![feature(get_many_mut)]
use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Node {
    number: i32,
    prev: usize,
    next: usize,
}

pub fn solution_a(input: &str) -> i32 {
    let mut nodes = parse_nodes(input);

    for i in 0..nodes.len() {
        move_ith_node(&mut nodes, i);
    }

    let start = nodes.iter().find(|n| n.number == 0).unwrap();
    let mut numbers = vec![];
    let mut idx = start.next;
    for i in 1..=3000 {
        let n = nodes[idx];
        idx = n.next;
        if i % 1000 == 0 {
            numbers.push(n.number);
        }
    }
    println!("{numbers:?}");
    numbers.iter().sum()
}

fn move_ith_node(nodes: &mut [Node], i: usize) {
    let current = nodes[i];
    if current.number == 0 {
        return;
    }

    let dest_idx = get_destination_idx(i, current, nodes);
    unsafe {
        let [prev, next] = nodes.get_many_unchecked_mut([current.prev, current.next]);
        prev.next = current.next;
        next.prev = current.prev;
    }

    let dest = nodes[dest_idx];
    unsafe {
        let [dest, current, dest_next] = nodes.get_many_unchecked_mut([dest_idx, i, dest.next]);
        current.prev = dest_idx;
        current.next = dest.next;
        dest.next = i;
        dest_next.prev = i;
    }
}

fn get_destination_idx(i: usize, current: Node, nodes: &mut [Node]) -> usize {
    let mut dest_idx = i;
    let n_iter = if current.number.is_positive() {
        current.number.abs()
    } else {
        current.number.abs() + 1
    };
    for _ in 0..n_iter {
        dest_idx = match current.number {
            n if n > 0 => nodes[dest_idx].next,
            n if n < 0 => nodes[dest_idx].prev,
            _ => i,
        };
    }
    dest_idx
}

fn parse_nodes(input: &str) -> Vec<Node> {
    let (_, numbers) = parse(input).unwrap();
    numbers
        .iter()
        .enumerate()
        .map(|(i, &number)| Node {
            number,
            prev: if i == 0 { numbers.len() - 1 } else { i - 1 },
            next: (i + 1) % numbers.len(),
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(newline, complete::i32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 3);
        println!("{}", solution_a(INPUT));
        // assert_eq!(solution_b(TEST_INPUT), 58);
        // println!("{}", solution_b(INPUT));
    }
}
