use aoc24::input::get_input;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    File(usize),
    Empty,
}


fn get_checksum(disk_map: &[Block]) -> usize {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| {
            if let Block::File(id) = b {
                Some(i * id)
            } else {
                None
            }
        })
        .sum()
}

fn part_one(disk_map: &mut [Block]) -> usize {
    for i in (0..disk_map.len()).rev() {
        if let Block::File(_) = disk_map[i] {
            if let Some(j) = disk_map.iter().take(i).position(|&b| b == Block::Empty) {
                disk_map.swap(i, j);
            }
        }
    }

    get_checksum(disk_map)
}

fn part_two(disk_map: &mut [Block]) -> usize {
    let unique_ids = disk_map
        .iter()
        .filter_map(|&b| {
            if let Block::File(id) = b {
                Some(id)
            } else {
                None
            }
        })
        .unique()
        .collect_vec();

    for &id in unique_ids.iter().rev() {
        let id_positions: Vec<_> = disk_map
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b == Block::File(id) { Some(i) } else { None })
            .collect();

        let length = id_positions.len();

        // Find first contiguous empty blocks with the same length as the file with id `id`
        if let Some(i) = disk_map
            .windows(length)
            .take(id_positions[0])
            .position(|w| w.iter().all(|&b| b == Block::Empty))
        {
            disk_map[i..i + length].fill(Block::File(id));
            disk_map[id_positions[0]..id_positions[0]+length].fill(Block::Empty);
        }
    }

    get_checksum(disk_map)
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut disk_map = Vec::new();
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .for_each(|(i, size)| {
            let id = if i % 2 == 0 {
                Block::File(i / 2)
            } else {
                Block::Empty
            };
            disk_map.extend(vec![id; size]);
        });
    disk_map
}
fn main() {
    let input = get_input(9);
    let mut disk_map = parse_input(&input);
    println!("Part one: {}", part_one(&mut disk_map.clone()));
    println!("Part two: {}", part_two(&mut disk_map));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc24::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(9);
        let mut disk_map = parse_input(&input);
        assert_eq!(part_one(&mut disk_map), 1928);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(9);
        let mut disk_map = parse_input(&input);
        assert_eq!(part_two(&mut disk_map), 2858);
    }
}
