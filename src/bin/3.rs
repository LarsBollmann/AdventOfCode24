use aoc24::input::{get_example, get_input};
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Mul(usize, usize),
    Dont,
    Do,
}

fn parse_muls(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|(do\(\))").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
                return Instruction::Mul(a.as_str().parse().unwrap(), b.as_str().parse().unwrap());
            }
            if cap.get(3).is_some() {
                return Instruction::Dont;
            }
            if cap.get(4).is_some() {
                return Instruction::Do;
            }
            unreachable!("No match")
        })
        .collect()
}

fn part_one(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

fn part_two(instructions: &[Instruction]) -> usize {
    let mut currently_dont = false;
    let mut result = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Dont => {
                currently_dont = true;
            }
            Instruction::Do => {
                currently_dont = false;
            }
            Instruction::Mul(a, b) => {
                if !currently_dont {
                    result += a * b;
                }
            }
        }
    }
    result
}

fn main() {
    let binding = get_input(3);
    let input = parse_muls(&binding);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(3);
        let parsed = parse_muls(&input);
        assert_eq!(part_one(&parsed), 161);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(3);
        let parsed = parse_muls(&input);
        assert_eq!(part_two(&parsed), 48);
    }
}
