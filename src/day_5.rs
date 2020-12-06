use std::cmp;

pub fn part_one(input: &'static str) -> Option<usize> {
    seat_ids(input).max()
}

pub fn part_two(input: &'static str) -> Option<usize> {
    let (ids, min, max) =
        seat_ids(input).fold((vec![], usize::MAX, 0), |(mut ids, min, max), id| {
            ids.push(id);
            (ids, cmp::min(min, id), cmp::max(max, id))
        });

    let mut seen = vec![false; (max - min) + 1];
    for id in ids {
        seen[id as usize - min] = true;
    }

    seen.iter().position(|seen| !seen).map(|idx| idx + min)
}

fn seat_ids(input: &'static str) -> impl Iterator<Item = usize> {
    input.split_terminator("\n").map(|pass| {
        // Pass is basically binary, convert 'B's and 'R's to 1s and others to 0s
        pass.chars()
            .fold(0, |acc, c| (acc << 1) + matches!(c, 'B' | 'R') as usize)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_5.txt");

    #[test]
    fn examples() {
        assert_eq!(seat_ids("FBFBBFFRLR").next(), Some(357));
        assert_eq!(seat_ids("BFFFBBFRRR").next(), Some(567));
        assert_eq!(seat_ids("FFFBBBFRRR").next(), Some(119));
        assert_eq!(seat_ids("BBFFBBFRLL").next(), Some(820));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT), Some(842));
        assert_eq!(part_two(INPUT), Some(617));
    }
}
