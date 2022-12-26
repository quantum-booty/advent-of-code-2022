#![feature(get_many_mut)]
use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Node {
    number: i32,
    idx: usize,
    is_head: bool,
    prev: usize,
    next: usize,
}

// when head moves, head.next becomes the head
pub fn solution_a(input: &str) -> i32 {
    let mut nodes = parse_nodes(input);

    // visualise(&nodes);
    for i in 0..nodes.len() {
        move_ith_node(&mut nodes, i);
        // println!("{nodes:?}");
        // visualise(&nodes);
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

fn visualise(nodes: &Vec<Node>) {
    let head_position = nodes.iter().position(|n| n.is_head).unwrap();
    let head = nodes[head_position];
    let mut numbers = vec![head.number];
    let mut cur_node = nodes[head.next];
    while !cur_node.is_head {
        numbers.push(cur_node.number);
        cur_node = nodes[cur_node.next];
    }
    println!("{numbers:?}");
}

fn move_ith_node(nodes: &mut [Node], i: usize) {
    let curr = nodes[i];
    if curr.number == 0 {
        // println!("{} does not move", current.number);
        return;
    }

    let dest_idx = get_destination_idx(i, curr, nodes);
    let dest = nodes[dest_idx];
    unsafe {
        let [curr_prev, curr, curr_next, dest, dest_next] =
            nodes.get_many_unchecked_mut([curr.prev, i, curr.next, dest_idx, dest.next]);

        curr_prev.next = curr_next.idx;
        curr_next.prev = curr_prev.idx;
        if curr.is_head {
            curr_next.is_head = true;
            curr.is_head = false;
        }

        dest.next = curr.idx;
        curr.prev = dest.idx;
        curr.next = dest_next.idx;
        dest_next.prev = curr.idx;
    }
}

fn get_destination_idx(i: usize, current: Node, nodes: &[Node]) -> usize {
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
            _ => unreachable!(),
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
            idx: i,
            is_head: i == 0,
            prev: if i == 0 { numbers.len() - 1 } else { i - 1 },
            next: if i == numbers.len() - 1 { 0 } else { i + 1 },
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
