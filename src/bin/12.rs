use aoc24::input::get_input;
use itertools::Itertools;
use std::collections::HashSet;

fn get_neighbors(i: usize, j: usize, garden: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if i < garden.len() - 1 {
        neighbors.push((i + 1, j));
    }
    if j < garden[i].len() - 1 {
        neighbors.push((i, j + 1));
    }
    neighbors
}

fn flood_fill(
    i: usize,
    j: usize,
    garden: &[Vec<char>],
    already_assigned: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut area = Vec::new();
    let mut stack = vec![(i, j)];
    while let Some((i, j)) = stack.pop() {
        if already_assigned.contains(&(i, j)) {
            continue;
        }
        already_assigned.insert((i, j));
        area.push((i, j));
        for (ni, nj) in get_neighbors(i, j, garden) {
            if garden[ni][nj] == garden[i][j] {
                stack.push((ni, nj));
            }
        }
    }
    area
}

fn is_border(i: usize, j: usize, di: i32, dj: i32, garden: &[Vec<char>]) -> bool {
    let ni = i as i32 + di;
    let nj = j as i32 + dj;
    if ni < 0 || nj < 0 {
        return true;
    }

    let ni = ni as usize;
    let nj = nj as usize;

    if ni >= garden.len() || nj >= garden[ni].len() {
        return true;
    }

    garden[ni][nj] != garden[i][j]
}

fn get_areas(garden: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let mut part_of_area = HashSet::new();
    let mut areas = Vec::new();

    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            if part_of_area.contains(&(i, j)) {
                continue;
            }

            let area = flood_fill(i, j, garden, &mut part_of_area);
            areas.push(area);
        }
    }
    areas
}

fn calculate_perimeter(positions: &[(usize, usize)], garden: &[Vec<char>]) -> usize {
    positions
        .iter()
        .map(|(i, j)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter(|(di, dj)| is_border(*i, *j, *di, *dj, garden))
                .count()
        })
        .sum()
}

fn part_one(garden: &[Vec<char>]) -> usize {
    let areas = get_areas(garden);
    areas
        .iter()
        .map(|positions| {
            let surface = positions.len();
            let perimeter = calculate_perimeter(positions, garden);
            surface * perimeter
        })
        .sum()
}

fn count_segments(positions: &[(usize, usize)], garden: &[Vec<char>], is_row: bool) -> usize {
    // Get unique indices per row or column
    let unique_indices = if is_row {
        positions
            .iter()
            .map(|(i, _)| i)
            .unique()
            .collect::<Vec<_>>()
    } else {
        positions
            .iter()
            .map(|(_, j)| j)
            .unique()
            .collect::<Vec<_>>()
    };

    // For each unique index, get the segments
    unique_indices
        .iter()
        .map(|index| {
            // Get all column indexes for a given row index and vice versa
            let line: Vec<_> = positions
                .iter()
                .filter_map(|(i, j)| {
                    if (is_row && i == *index) || (!is_row && j == *index) {
                        Some(if is_row { j } else { i })
                    } else {
                        None
                    }
                })
                .collect();

            [-1, 1]
                .iter()
                .map(|d| {
                    let border: Vec<_> = line
                        .iter()
                        .filter(|&pos| {
                            if is_row {
                                is_border(**index, **pos, *d, 0, garden)
                            } else {
                                is_border(**pos, **index, 0, *d, garden)
                            }
                        })
                        .collect();

                    let mut segments = 1usize;
                    if border
                        .iter()
                        .sorted()
                        .reduce(|acc, e| {
                            if **e - **acc > 1 {
                                segments += 1;
                            }
                            e
                        })
                        .is_none()
                    {
                        segments = 0;
                    }
                    segments
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part_two(garden: &[Vec<char>]) -> usize {
    let areas = get_areas(garden);
    areas
        .iter()
        .map(|positions| {
            let segments_top_bottom = count_segments(positions, garden, true);
            let segments_left_right = count_segments(positions, garden, false);
            let surface = positions.len();
            let perimeter = segments_top_bottom + segments_left_right;
            surface * perimeter
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn main() {
    let input = get_input(12);
    let garden = parse_input(&input);
    println!("Part one: {}", part_one(&garden));
    println!("Part two: {}", part_two(&garden));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(12);
        let garden = parse_input(&input);
        assert_eq!(part_one(&garden), 1930);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(12);
        let garden = parse_input(&input);
        assert_eq!(part_two(&garden), 1206);
    }
}
