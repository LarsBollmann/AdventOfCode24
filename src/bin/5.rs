use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

use aoc24::input::get_input;

#[derive(Clone)]
struct Page {
    pub number: usize,
    pub must_be_printed_before: Vec<usize>,
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Page {}

impl PartialEq<usize> for Page {
    fn eq(&self, other: &usize) -> bool {
        self.number == *other
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.must_be_printed_before.contains(&other.number) {
            Ordering::Less
        } else if other.must_be_printed_before.contains(&self.number) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (HashMap<usize, Page>, Vec<Vec<usize>>) {
    let mut pages: HashMap<usize, Page> = HashMap::new();

    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    rules.lines().for_each(|rule| {
        if let Some((left, right)) = rule.split_once('|') {
            let left = left.trim().parse().unwrap();
            let right = right.trim().parse().unwrap();
            pages
                .entry(left)
                .or_insert_with(|| Page {
                    number: left,
                    must_be_printed_before: vec![],
                })
                .must_be_printed_before
                .push(right);
        }
    });
    let updates = parts.next().unwrap();
    let updates = updates
        .lines()
        .map(|update| update.split(',').map(|num| num.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    (pages, updates)
}

fn part_one(pages: &HashMap<usize, Page>, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            let update_pages: Vec<_> = update.iter().map(|num| pages.get(num).unwrap()).collect();
            // Check if update is sorted
            match update_pages.windows(2).all(|window| window[0] <= window[1]) {
                true => Some(update_pages[update_pages.len() / 2].number),
                false => None,
            }
        })
        .sum()
}

fn part_two(pages: &HashMap<usize, Page>, updates: &mut [Vec<usize>]) -> usize {
    updates
        .iter_mut()
        .filter_map(|update| {
            let mut update_pages: Vec<_> = update
                .iter()
                .map(|num| pages.get(num).unwrap().clone())
                .collect();
            // Check if update is sorted
            match update_pages.windows(2).all(|window| window[0] <= window[1]) {
                true => None,
                false => {
                    update_pages.sort();
                    Some(update_pages[update_pages.len() / 2].number)
                }
            }
        })
        .sum()
}

fn main() {
    let input = get_input(5);
    let (pages, updates) = parse_input(&input);

    println!("Part one: {}", part_one(&pages, &updates));
    println!("Part two: {}", part_two(&pages, &mut updates.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(5);
        let (pages, updates) = parse_input(&input);
        let result = part_one(&pages, &updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(5);
        let (pages, mut updates) = parse_input(&input);
        let result = part_two(&pages, &mut updates);
        assert_eq!(result, 123);
    }
}
