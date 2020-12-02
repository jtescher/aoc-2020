const INPUT: &'static str = include_str!("../data/day_1.txt");

pub fn part_one() -> anyhow::Result<Option<u64>> {
    for (i, first) in INPUT.split_terminator("\n").enumerate() {
        let first: u64 = first.parse()?;

        for second in INPUT.split_terminator("\n").skip(i) {
            let second: u64 = second.parse()?;

            if first + second == 2020 {
                return Ok(Some(first * second));
            }
        }
    }

    Ok(None)
}

pub fn part_two() -> anyhow::Result<Option<u64>> {
    for (i, first) in INPUT.split_terminator("\n").enumerate() {
        let first: u64 = first.parse()?;

        for (j, second) in INPUT.split_terminator("\n").skip(i).enumerate() {
            let second: u64 = second.parse()?;

            for third in INPUT.split_terminator("\n").skip(i + j) {
                let third: u64 = third.parse()?;

                if first + second + third == 2020 {
                    return Ok(Some(first * second * third));
                }
            }
        }
    }

    Ok(None)
}
