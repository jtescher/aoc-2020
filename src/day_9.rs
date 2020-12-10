pub fn part_one(input: &str, preamble_len: usize) -> anyhow::Result<Option<usize>> {
    let inputs = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    let res = inputs
        .iter()
        .enumerate()
        .skip(preamble_len)
        .find(|(idx, &x)| {
            let window = &inputs[idx - preamble_len..*idx];
            !window.iter().any(|&w| w < x && window.contains(&(x - w)))
        })
        .map(|(_idx, &x)| x);

    Ok(res)
}

pub fn part_two(input: &str, target_number: usize) -> anyhow::Result<Option<usize>> {
    let inputs = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    for start_idx in 0..inputs.len() {
        let mut current_sum = inputs[start_idx];
        let mut end_idx = start_idx;
        let mut smallest = current_sum;
        let mut largest = current_sum;

        while current_sum < target_number {
            end_idx += 1;
            current_sum += inputs[end_idx];
            smallest = smallest.min(inputs[end_idx]);
            largest = largest.max(inputs[end_idx]);

            if current_sum == target_number {
                return Ok(Some(smallest + largest));
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_9.txt");

    #[test]
    fn examples() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        assert_eq!(part_one(input, 5).unwrap(), Some(127));
        assert_eq!(part_two(input, 127).unwrap(), Some(62));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT, 25).unwrap(), Some(18_272_118));
        assert_eq!(part_two(INPUT, 18_272_118).unwrap(), Some(2_186_361));
    }
}
