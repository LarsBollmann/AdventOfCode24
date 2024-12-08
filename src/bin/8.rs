use std::collections::{HashMap, HashSet};

use aoc24::input::get_input;
use glam::IVec2;
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<IVec2>>,
}

fn parse_input(input: &str) -> Map {
    let mut antennas = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height = y + 1;
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(IVec2::new(x as i32, y as i32));
            }
        }
    }
    Map {
        width,
        height,
        antennas,
    }
}

fn is_in_bounds(antinode: IVec2, width: usize, height: usize) -> bool {
    (0..width as i32).contains(&antinode.x) && (0..height as i32).contains(&antinode.y)
}

fn get_antinodes_in_direction(
    antenna: IVec2,
    diff: IVec2,
    width: usize,
    height: usize,
) -> Vec<IVec2> {
    let mut antinodes = Vec::new();
    let mut antinode = antenna + diff;
    while is_in_bounds(antinode, width, height) {
        antinodes.push(antinode);
        antinode += diff;
    }
    antinodes
}

fn solve(map: &Map, part_one: bool) -> usize {
    map.antennas
        .values()
        .flat_map(|antennas: &Vec<IVec2>| {
            antennas.iter().tuple_combinations().flat_map(|(a, b)| {
                let diff = *a - *b;
                if part_one {
                    let antinode1 = *a + diff;
                    let antinode2 = *b - diff;
                    return vec![antinode1, antinode2]
                        .into_iter()
                        .filter(|antinode| is_in_bounds(*antinode, map.width, map.height))
                        .collect::<Vec<_>>();
                }
                let mut antinodes = vec![*a, *b];
                antinodes.extend(get_antinodes_in_direction(*a, diff, map.width, map.height));
                antinodes.extend(get_antinodes_in_direction(*b, -diff, map.width, map.height));
                antinodes
            })
        })
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let input = get_input(8);
    let map = parse_input(&input);
    println!("Part One: {}", solve(&map, true));
    println!("Part Two: {}", solve(&map, false));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(8);
        let map = parse_input(&input);
        assert_eq!(solve(&map, true), 14);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(8);
        let map = parse_input(&input);
        assert_eq!(solve(&map, false), 34);
    }
}
