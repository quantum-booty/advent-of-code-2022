pub fn solution(input: &str) -> String {
    let snafus = input.lines().map(to_decimal).collect::<Vec<i64>>();
    let sum = snafus.iter().sum();
    to_snafu(sum)
}

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |decimal, c| {
        decimal * 5
            + match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '=' => -2,
                '-' => -1,
                _ => unreachable!(),
            }
    })
}

fn to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        return String::new();
    }
    let remainder = decimal % 5;
    to_snafu((decimal + 2) / 5)
        + match remainder {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "=",
            4 => "-",
            _ => unreachable!(),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solution(TEST_INPUT), "2=-1=0");
        println!("{}", solution(INPUT));
    }

    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    const INPUT: &str = include_str!("input.txt");
}
