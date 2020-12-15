use std::collections::HashMap;

pub fn nth_spoken(input: &[usize], target: usize) -> usize {
    let mut state = input
        .iter()
        .enumerate()
        .map(|(idx, &n)| (n, idx + 1))
        .collect::<HashMap<usize, usize>>();

    (input.len()..target).fold(*input.last().unwrap(), |last, turn| {
        let current = state.get(&last).map_or(0, |prev| turn - prev);
        state.insert(last, turn);

        current
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(nth_spoken(&[0, 3, 6], 2020), 436);
        assert_eq!(nth_spoken(&[1, 3, 2], 2020), 1);
        assert_eq!(nth_spoken(&[2, 1, 3], 2020), 10);
        assert_eq!(nth_spoken(&[1, 2, 3], 2020), 27);
        assert_eq!(nth_spoken(&[2, 3, 1], 2020), 78);
        assert_eq!(nth_spoken(&[3, 2, 1], 2020), 438);
        assert_eq!(nth_spoken(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(nth_spoken(&[0, 3, 6], 30_000_000), 175_594);
    }

    #[test]
    fn real_input() {
        assert_eq!(nth_spoken(&[20, 9, 11, 0, 1, 2], 2020), 1111);
        assert_eq!(nth_spoken(&[20, 9, 11, 0, 1, 2], 30_000_000), 48_568);
    }
}
