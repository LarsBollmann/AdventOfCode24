use std::collections::HashMap;

use aoc24::input::get_input;

pub fn part_one(a: &mut [usize], b: &mut Vec<usize>) -> usize {
    a.sort();
    b.sort();

    a.iter_mut()
        .zip(b)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn get_hashmap(a: &[usize]) -> HashMap<usize, usize> {
    a.iter().fold(HashMap::new(), |mut map, el| {
        if let Some(count) = map.get_mut(el) {
            *count += 1;
        } else {
            map.insert(*el, 1);
        }
        map
    })
}

pub fn part_two(a: &[usize], b: &[usize]) -> usize {
    let hashmap_b = get_hashmap(b);

    a.iter().fold(0, |acc, el| {
        if let Some(count) = hashmap_b.get(el) {
            acc + el * count
        } else {
            acc
        }
    })
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines();
    lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a: usize = parts.next().unwrap().parse().unwrap();
            let b: usize = parts.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip()
}
fn main() {
    let input = get_input(1);
    let (mut a, mut b) = parse_input(&input);
    println!("Part one: {}", part_one(&mut a, &mut b));
    println!("Part two: {}", part_two(&a, &b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(1);
        let (mut a, mut b) = parse_input(&input);
        let distances = part_one(&mut a, &mut b);
        assert_eq!(distances, 11);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(1);
        let (a, b) = parse_input(&input);
        let result = part_two(&a, &b);
        assert_eq!(result, 31);
    }
}
