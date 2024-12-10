use aoc24::input::get_input;
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    map: Vec<Vec<usize>>,
}

type Position = (usize, usize);

fn get_zero_positions(map: &Map) -> impl Iterator<Item = Position> + '_ {
    map.map.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(j, &v)| if v == 0 { Some((i, j)) } else { None })
    })
}

fn part_one(map: &Map) -> usize {
    get_zero_positions(map)
        .map(|zero_position| {
            get_paths(map, zero_position)
                .iter()
                .map(|p| p.last().unwrap())
                .unique()
                .count()
        })
        .sum()
}

fn part_two(map: &Map) -> usize {
    get_zero_positions(map)
        .map(|zero_position| get_paths(map, zero_position).len())
        .sum()
}

fn get_paths(map: &Map, start: Position) -> Vec<Vec<Position>> {
    let mut paths = vec![];
    let mut stack = vec![(vec![start], start)];

    while let Some((path, pos)) = stack.pop() {
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if new_pos.0 >= map.map.len() || new_pos.1 >= map.map[0].len() {
                continue;
            }
            if path.contains(&new_pos) {
                continue;
            }
            if map.map[new_pos.0][new_pos.1] != map.map[pos.0][pos.1] + 1 {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(new_pos);
            if map.map[new_pos.0][new_pos.1] == 9 {
                paths.push(new_path);
            } else {
                stack.push((new_path, new_pos));
            }
        }
    }
    paths
}

fn parse_input(input: &str) -> Map {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    Map { map }
}
fn main() {
    let input = get_input(10);
    let map = parse_input(&input);
    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(10);
        let map = parse_input(&input);
        assert_eq!(part_one(&map), 36);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(10);
        let map = parse_input(&input);
        assert_eq!(part_two(&map), 81);
    }
}
