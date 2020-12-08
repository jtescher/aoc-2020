use anyhow::anyhow;

pub fn part_one(input: &str) -> anyhow::Result<u32> {
    let mut result = 0;

    for line in input.lines() {
        let (min, max, rule_char, pswd) = extract_parts(line)?;

        if (min..=max).contains(&pswd.chars().filter(|c| *c == rule_char).count()) {
            result += 1;
        }
    }

    Ok(result)
}

pub fn part_two(input: &str) -> anyhow::Result<u32> {
    let mut result = 0;

    for line in input.lines() {
        let (first, second, rule_char, pswd) = extract_parts(line)?;

        let first_match = pswd.chars().nth(first - 1).filter(|&c| c == rule_char);
        let second_match = pswd.chars().nth(second - 1).filter(|&c| c == rule_char);

        if first_match.xor(second_match).is_some() {
            result += 1;
        }
    }

    Ok(result)
}

fn extract_parts(line: &str) -> anyhow::Result<(usize, usize, char, &str)> {
    let mut parts = line.split_whitespace();
    let (first, second) = parts
        .next()
        .and_then(|first_second| {
            let mut parts = first_second.split_terminator('-');
            if let (Some(first), Some(second)) = (
                parts.next().and_then(|part| part.parse().ok()),
                parts.next().and_then(|part| part.parse().ok()),
            ) {
                Some((first, second))
            } else {
                None
            }
        })
        .ok_or(anyhow!("expected first-second range in {}", line))?;

    let rule_char = parts
        .next()
        .and_then(|letter| letter.chars().next())
        .ok_or(anyhow!("expected rule character in {}", line))?;

    let pswd = parts
        .next()
        .ok_or(anyhow!("expected password in {}", line))?;

    Ok((first, second, rule_char, pswd))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_2.txt");

    #[test]
    fn examples() {
        assert_eq!(extract_parts("1-3 a: abcde").unwrap(), (1, 3, 'a', "abcde"))
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT).unwrap(), 424);
        assert_eq!(part_two(INPUT).unwrap(), 747);
    }
}
