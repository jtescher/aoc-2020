use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let mut addresses = HashMap::new();
    let mut mask = "";

    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line
                .splitn(2, " = ")
                .last()
                .ok_or(anyhow::anyhow!("expected mask, got {}", line))?;
        } else {
            let mut parts = line.split_terminator(" = ");
            let addr = parts
                .next()
                .and_then(|addr| {
                    addr.trim_start_matches("mem[")
                        .trim_end_matches("]")
                        .parse::<usize>()
                        .ok()
                })
                .ok_or(anyhow::anyhow!("expected addr, got {}", line))?;
            let mut val = parts
                .next()
                .and_then(|val| val.parse::<usize>().ok())
                .ok_or(anyhow::anyhow!("expected mem val, got {}", line))?;

            for (idx, c) in mask.chars().rev().enumerate() {
                if c == '1' {
                    val = val | (1 << idx);
                } else if c == '0' {
                    val = val & !(1 << idx);
                }
            }

            addresses.insert(addr, val);
        }
    }

    Ok(addresses.values().sum())
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let mut addresses = HashMap::new();
    let mut mask = "";

    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line
                .splitn(2, " = ")
                .last()
                .ok_or(anyhow::anyhow!("expected mask, got {}", line))?;
        } else {
            let mut parts = line.split_terminator(" = ");
            let mut addr = parts
                .next()
                .and_then(|addr| {
                    addr.trim_start_matches("mem[")
                        .trim_end_matches("]")
                        .parse::<usize>()
                        .ok()
                })
                .ok_or(anyhow::anyhow!("expected addr, got {}", line))?;
            let val = parts
                .next()
                .and_then(|val| val.parse::<usize>().ok())
                .ok_or(anyhow::anyhow!("expected mem val, got {}", line))?;

            let mut floating: Vec<Vec<(usize, usize)>> = vec![];
            for (idx, c) in mask.chars().rev().enumerate() {
                if c == '1' {
                    addr = addr | (1 << idx);
                } else if c == 'X' {
                    if floating.is_empty() {
                        floating.push(vec![(idx, 0)]);
                        floating.push(vec![(idx, 1)]);
                    } else {
                        let mut floating2 = floating.clone();
                        for i in &mut floating {
                            i.push((idx, 0));
                        }
                        for j in &mut floating2 {
                            j.push((idx, 1));
                        }

                        floating.append(&mut floating2);
                    }
                }
            }
            addresses.insert(addr, val);

            for addr_variant in floating {
                let mut new_addr = addr.clone();
                for (idx, num) in addr_variant {
                    if num == 1 {
                        new_addr = new_addr | (1 << idx);
                    } else {
                        new_addr = new_addr & !(1 << idx);
                    }
                }
                addresses.insert(new_addr, val);
            }
        }
    }

    Ok(addresses.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(part_one(input).unwrap(), 165);
        let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part_two(input).unwrap(), 208);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../data/day_14.txt");

        assert_eq!(part_one(input).unwrap(), 8570568288597);
        assert_eq!(part_two(input).unwrap(), 3289441921203);
    }
}
