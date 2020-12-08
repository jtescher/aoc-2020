pub fn part_one(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .fold(0u32, |bits, c| bits | 1 << (c as u32 - 'a' as u32))
                .count_ones()
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|member| {
                    member
                        .chars()
                        .fold(0u32, |bits, c| bits | 1 << (c as u32 - 'a' as u32))
                })
                .fold(u32::MAX, |acc, bits| acc & bits)
                .count_ones()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_6.txt");

    #[test]
    fn examples() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part_one(input), 11);
        assert_eq!(part_two(input), 6);
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT), 6947);
        assert_eq!(part_two(INPUT), 3398);
    }
}
