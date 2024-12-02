use aoc24::input::get_input;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(row: &[isize]) -> bool {
    let diffs: Vec<_> = row.windows(2).map(|pair| pair[1] - pair[0]).collect();
    diffs.iter().all(|diff| (1..=3).contains(diff))
        || diffs.iter().all(|diff| (-3..=-1).contains(diff))
}

fn part_one(input: &[Vec<isize>]) -> usize {
    input.iter().filter(|row| is_valid(row)).count()
}

fn part_two(input: &[Vec<isize>]) -> usize {
    input
        .iter()
        .filter(|row| {
            for i in 0..row.len() {
                let mut row_one_removed = row.to_vec();
                row_one_removed.remove(i);

                if is_valid(&row_one_removed) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn main() {
    let input = parse_input(&get_input(2));
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(2);
        let parsed = parse_input(&input);
        let result = part_one(&parsed);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(2);
        let parsed = parse_input(&input);
        let result = part_two(&parsed);
        assert_eq!(result, 4);
    }
}
