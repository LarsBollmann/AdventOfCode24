use aoc24::input::get_input;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn operate(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

fn get_possible_combinations(
    options: &[Operator],
    len: usize,
) -> impl Iterator<Item = Vec<&Operator>> {
    (0..len).map(|_| options).multi_cartesian_product()
}

fn solve(available_operators: Vec<Operator>, equations: &[Equation]) -> usize {
    equations
        .par_iter()
        .fold(
            || 0,
            |acc, equation| {
                let mut possible_combinations =
                    get_possible_combinations(&available_operators, equation.numbers.len() - 1);

                if possible_combinations.any(|operators| {
                    let result = equation
                        .numbers
                        .iter()
                        .skip(1)
                        .zip(operators.iter())
                        .fold(equation.numbers[0], |acc, (number, operator)| {
                            operator.operate(acc, *number)
                        });
                    result == equation.result
                }) {
                    acc + equation.result
                } else {
                    acc
                }
            },
        )
        .sum()
}

fn part_one(equations: &[Equation]) -> usize {
    let available_operators = vec![Operator::Add, Operator::Multiply];
    solve(available_operators, equations)
}

fn part_two(equations: &[Equation]) -> usize {
    let available_operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
    solve(available_operators, equations)
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, number_str) = line.split_once(": ").unwrap();
            let numbers = number_str.split(' ').map(|n| n.parse().unwrap()).collect();
            Equation {
                result: result.parse().unwrap(),
                numbers,
            }
        })
        .collect()
}
fn main() {
    let input = get_input(7);
    let equations = parse_input(&input);
    println!("Part one: {}", part_one(&equations));
    println!("Part two: {}", part_two(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(7);
        let equations = parse_input(&input);
        let result = part_one(&equations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(7);
        let equations = parse_input(&input);
        let result = part_two(&equations);
        assert_eq!(result, 11387);
    }
}
