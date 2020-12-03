const INPUT: &'static str = include_str!("../data/day_3.txt");

pub fn part_one() -> u32 {
    run_slope(3, 1)
}

pub fn part_two() -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| run_slope(right, down))
        .product()
}

fn run_slope(right: usize, down: usize) -> u32 {
    INPUT
        .split_terminator("\n")
        .enumerate()
        .step_by(down)
        .fold(0, |acc, (idx, line)| {
            if line.chars().nth((idx * right) % line.len()) == Some('#') {
                acc + 1
            } else {
                acc
            }
        })
}
