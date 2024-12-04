use aoc24::input::get_input;

fn check_direction(x: usize, y: usize, dx: isize, dy: isize, input: &[Vec<char>]) -> bool {
    const SEQUENCE: &[char] = &['X', 'M', 'A', 'S'];
    let positions: Vec<_> = (0..SEQUENCE.len())
        .map(|i| (x as isize + i as isize * dx, y as isize + i as isize * dy))
        .collect();

    if positions.iter().any(|&(nx, ny)| {
        nx < 0 || ny < 0 || nx >= input.len() as isize || ny >= input[0].len() as isize
    }) {
        return false;
    }

    SEQUENCE
        .iter()
        .enumerate()
        .all(|(i, c)| input[positions[i].0 as usize][positions[i].1 as usize] == *c)
}

fn check_all_directions(x: usize, y: usize, input: &[Vec<char>]) -> usize {
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    directions
        .iter()
        .filter(|&&(dx, dy)| check_direction(x, y, dx, dy, input))
        .count()
}

fn part_one(input: &[Vec<char>]) -> usize {
    input.iter().enumerate().fold(0, |acc, (x, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (y, c)| {
            if *c == 'X' {
                return acc + check_all_directions(x, y, input);
            }
            acc
        })
    })
}

fn part_two(input: &[Vec<char>]) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();

    (1..num_rows - 1).fold(0, |acc, x| {
        acc + (1..num_cols - 1).fold(0, |acc, y| {
            let c = input[x][y];
            if c == 'A' {
                let diagonals = [
                    ((x - 1, y - 1), (x + 1, y + 1)),
                    ((x - 1, y + 1), (x + 1, y - 1)),
                ];
                if diagonals.iter().all(|&((x1, y1), (x2, y2))| {
                    (input[x1][y1] == 'M' && input[x2][y2] == 'S')
                        || (input[x1][y1] == 'S' && input[x2][y2] == 'M')
                }) {
                    return acc + 1;
                }
            }
            acc
        })
    })
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let input = get_input(4);
    let parsed = parse_input(&input);
    println!("Part one: {}", part_one(&parsed));
    println!("Part two: {}", part_two(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(4);
        let parsed = parse_input(&input);
        let result = part_one(&parsed);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(4);
        let parsed = parse_input(&input);
        let result = part_two(&parsed);
        assert_eq!(result, 9);
    }
}
