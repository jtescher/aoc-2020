pub fn part_one(input: &str) -> anyhow::Result<Option<u64>> {
    for (i, first) in input.lines().enumerate() {
        let first: u64 = first.parse()?;

        for second in input.lines().skip(i) {
            let second: u64 = second.parse()?;

            if first + second == 2020 {
                return Ok(Some(first * second));
            }
        }
    }

    Ok(None)
}

pub fn part_two(input: &str) -> anyhow::Result<Option<u64>> {
    for (i, first) in input.lines().enumerate() {
        let first: u64 = first.parse()?;

        for (j, second) in input.lines().skip(i).enumerate() {
            let second: u64 = second.parse()?;

            for third in input.lines().skip(i + j) {
                let third: u64 = third.parse()?;

                if first + second + third == 2020 {
                    return Ok(Some(first * second * third));
                }
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_1.txt");

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT).unwrap().unwrap(), 224436);
        assert_eq!(part_two(INPUT).unwrap().unwrap(), 303394260);
    }
}
