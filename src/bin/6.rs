use std::collections::HashSet;

use aoc24::input::get_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_clockwise(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn update_position(&self, y: usize, x: usize, y_max: usize, x_max: usize) -> Option<Position> {
        match self {
            Direction::Up => {
                if y > 0 {
                    Some((y - 1, x))
                } else {
                    None
                }
            }
            Direction::Down => {
                if y < y_max {
                    Some((y + 1, x))
                } else {
                    None
                }
            }
            Direction::Left => {
                if x > 0 {
                    Some((y, x - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if x < x_max {
                    Some((y, x + 1))
                } else {
                    None
                }
            }
        }
    }
}

type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>, guard: Guard) -> Self {
        Map { tiles, guard }
    }

    fn update_guard_position(&mut self) -> Option<Position> {
        let (y, x) = self.guard.position;
        let (y_max, x_max) = (self.tiles.len() - 1, self.tiles[0].len() - 1);
        self.guard.direction.update_position(y, x, y_max, x_max)
    }

    fn step(&mut self) -> bool {
        if let Some(new_position) = self.update_guard_position() {
            if self.tiles[new_position.0][new_position.1] == Tile::Obstacle {
                self.guard.direction.rotate_clockwise();
            } else {
                self.guard.position = new_position;
            }
            true
        } else {
            false
        }
    }
}

fn part_one(map: &mut Map) -> usize {
    let mut visited = HashSet::new();
    visited.insert(map.guard.position);
    while map.step() {
        visited.insert(map.guard.position);
    }
    visited.len()
}

fn is_loop(map: &mut Map) -> bool {
    let mut visited = HashSet::new();
    visited.insert((map.guard.position, map.guard.direction));
    while map.step() {
        if visited.contains(&(map.guard.position, map.guard.direction)) {
            return true;
        }
        visited.insert((map.guard.position, map.guard.direction));
    }
    false
}

fn part_two(map: &mut Map) -> usize {
    let start_position = map.guard.position;
    let start_direction = map.guard.direction;

    let mut visited = HashSet::new();
    visited.insert((map.guard.position, map.guard.direction));
    while map.step() {
        visited.insert((map.guard.position, map.guard.direction));
    }

    visited
        .par_iter()
        .filter_map(|(pos, direction)| {
            let mut map = map.clone();
            map.guard.position = start_position;
            map.guard.direction = start_direction;
            let new_obstacle_position = direction.update_position(
                pos.0,
                pos.1,
                map.tiles.len() - 1,
                map.tiles[0].len() - 1,
            );
            if let Some(new_obstacle_position) = new_obstacle_position {
                if new_obstacle_position == start_position {
                    return None;
                }

                if map.tiles[new_obstacle_position.0][new_obstacle_position.1] == Tile::Empty {
                    map.tiles[new_obstacle_position.0][new_obstacle_position.1] = Tile::Obstacle;
                    if is_loop(&mut map) {
                        return Some(new_obstacle_position);
                    }
                }
            }
            None
        })
        .collect::<HashSet<_>>()
        .len()
}

fn parse_input(input: &str) -> Map {
    let mut guard_position = None;
    let tiles = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, tile)| match tile {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstacle,
                    '^' => {
                        guard_position = Some((i, j));
                        Tile::Empty
                    }
                    _ => panic!("Invalid tile!"),
                })
                .collect()
        })
        .collect();

    Map::new(
        tiles,
        Guard {
            position: guard_position.unwrap(),
            direction: Direction::Up,
        },
    )
}
fn main() {
    let input = get_input(6);
    let mut map = parse_input(&input);
    println!("Part one: {}", part_one(&mut map.clone()));
    println!("Part two: {}", part_two(&mut map));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(6);
        let mut map = parse_input(&input);
        let result = part_one(&mut map);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(6);
        let mut map = parse_input(&input);
        let result = part_two(&mut map);
        assert_eq!(result, 6);
    }
}
