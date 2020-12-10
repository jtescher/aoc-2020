pub fn part_one(input: &str) -> anyhow::Result<u32> {
    let mut inputs = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    inputs.sort_unstable();
    inputs.push(inputs.last().copied().unwrap_or(0) + 3);

    let mut current = 0;
    let mut ones_count = 0;
    let mut threes_count = 0;

    for input in inputs {
        let diff = input - current;
        if diff == 1 {
            ones_count += 1;
        } else if diff == 3 {
            threes_count += 1;
        }
        current += diff
    }

    Ok(ones_count * threes_count)
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let mut inputs = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    inputs.push(0);
    inputs.sort_unstable();
    inputs.push(inputs.last().copied().unwrap_or(0) + 3);

    let mut sums = vec![0; inputs.len()];
    sums[0] = 1;

    for (input_idx, input) in inputs.iter().enumerate() {
        for (output_idx, output) in inputs.iter().enumerate().skip(input_idx + 1) {
            if output - input > 3 {
                break;
            }

            sums[output_idx] += sums[input_idx];
        }
    }

    Ok(sums.last().copied().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_10.txt");

    #[test]
    fn examples() {
        let input = r"16
10
15
5
1
11
7
19
6
12
4
";

        assert_eq!(part_one(input).unwrap(), 35);
        assert_eq!(part_two(input).unwrap(), 8);

        let input_2 = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part_one(input_2).unwrap(), 220);
        assert_eq!(part_two(input_2).unwrap(), 19208);
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT).unwrap(), 2232);
        assert_eq!(part_two(INPUT).unwrap(), 173625106649344);
    }
}
