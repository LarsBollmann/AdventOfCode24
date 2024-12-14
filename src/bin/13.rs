use aoc24::input::get_input;
use glam::{DMat2, DVec2};

#[derive(Debug)]
struct Game {
    a: DVec2,
    b: DVec2,
    prize: DVec2,
}

fn solve(games: &[Game]) -> usize {
    games
        .iter()
        .filter_map(|game| {
            let matrix = DMat2::from_cols(game.a, game.b);
            let inv = matrix.inverse();
            let result = inv * game.prize;
            if (result.x - result.x.round()).abs() < 1e-4 && (result.y - result.y.round()).abs() < 1e-4 {
                Some((result * DVec2::new(3.0, 1.0)).element_sum())
            } else {
                None
            }
        })
        .sum::<f64>() as usize
}

fn part_one(games: &[Game]) -> usize {
    solve(games)
}

fn part_two(games: &[Game]) -> usize {
    // Add 10000000000000 to every prize coordinate
    let games = games
        .iter()
        .map(|game| Game {
            a: game.a,
            b: game.b,
            prize: game.prize + DVec2::new(10000000000000.0, 10000000000000.0),
        })
        .collect::<Vec<_>>();

    solve(&games)
}

fn parse_input(input: &str) -> Vec<Game> {
    let regexp = regex::Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    input
        .split("\n\n")
        .map(|block| {
            let results = regexp.captures_iter(block).collect::<Vec<_>>();
            let a = DVec2::new(
                results[0][1].parse().unwrap(),
                results[0][2].parse().unwrap(),
            );
            let b = DVec2::new(
                results[1][1].parse().unwrap(),
                results[1][2].parse().unwrap(),
            );
            let prize = DVec2::new(
                results[2][1].parse().unwrap(),
                results[2][2].parse().unwrap(),
            );
            Game { a, b, prize }
        })
        .collect()
}

fn main() {
    let input = get_input(13);
    let games = parse_input(&input);
    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(13);
        let games = parse_input(&input);
        assert_eq!(part_one(&games), 480);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(13);
        let games = parse_input(&input);
        assert_eq!(part_two(&games), 875318608908);
    }
}
