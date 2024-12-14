use core::str;
use std::collections::HashMap;

use aoc24::input::get_input;

#[derive(Debug, Clone, Copy)]
enum Action {
    Split(usize, usize),
    Multiply,
    AddOne,
}

fn act(number: usize) -> Action {
    let chars = number.to_string().chars().collect::<Vec<_>>();
    let len = chars.len();
    if len % 2 == 0 {
        let a = chars[..len / 2].iter().collect::<String>().parse().unwrap();
        let b = chars[len / 2..].iter().collect::<String>().parse().unwrap();
        Action::Split(a, b)
    } else if number != 0 {
        return Action::Multiply;
    } else {
        return Action::AddOne;
    }
}

fn solve(mut stones: HashMap<usize, usize>, steps: usize) -> usize {
    for _ in 0..steps {
        let mut new_stones = HashMap::new();
        for (number, count) in stones.iter() {
            match act(*number) {
                Action::Split(a, b) => {
                    *new_stones.entry(a).or_insert(0) += count;
                    *new_stones.entry(b).or_insert(0) += count;
                }
                Action::Multiply => {
                    *new_stones.entry(number * 2024).or_insert(0) += count;
                }
                Action::AddOne => {
                    *new_stones.entry(number + 1).or_insert(0) += count;
                }
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn part_one(stones: HashMap<usize, usize>) -> usize {
    solve(stones, 25)
}

fn part_two(stones: HashMap<usize, usize>) -> usize {
    solve(stones, 75)
}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    for number in input.split_whitespace() {
        let number = number.parse().unwrap();
        result.get_mut(&number).map(|v| *v += 1).unwrap_or_else(|| {
            result.insert(number, 1);
        });
    }
    result
}

fn main() {
    let input = get_input(11);
    let stones = parse_input(&input);
    println!("Part One: {}", part_one(stones.clone()));
    println!("Part Two: {}", part_two(stones));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(11);
        let stones = parse_input(&input);
        assert_eq!(part_one(stones), 55312);
    }
}
