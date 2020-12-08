pub fn part_one(input: &str) -> u32 {
    run_slope(3, 1, input)
}

pub fn part_two(input: &str) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| run_slope(right, down, input))
        .product()
}

fn run_slope(right: usize, down: usize, input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .step_by(down)
        .skip(1)
        .fold(0, |acc, (idx, line)| {
            if line.chars().nth((idx * right) % line.len()) == Some('#') {
                acc + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_3.txt");

    #[test]
    fn examples() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!(part_one(input), 7);
        assert_eq!(part_two(input), 168);
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT), 184);
        assert_eq!(part_two(INPUT), 2431272960);
    }
}
