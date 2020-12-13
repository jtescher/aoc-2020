pub fn part_one(input: &str) -> anyhow::Result<Option<usize>> {
    let mut lines = input.lines();

    let earliest_departure = lines
        .next()
        .and_then(|start| start.parse::<usize>().ok())
        .ok_or(anyhow::anyhow!("first line must be number"))?;

    lines
        .next()
        .map(|bus_ids| {
            bus_ids
                .split_terminator(',')
                .filter(|&id| id != "x")
                .filter_map(|id| id.parse::<usize>().ok())
                .map(|id| (id, id - (earliest_departure % id)))
                .min_by_key(|&(_id, wait_time)| wait_time)
                .map(|(id, wait_time)| id * wait_time)
        })
        .ok_or(anyhow::anyhow!("second line must be valid ids"))
}

pub fn part_two(input: &str) -> anyhow::Result<Option<usize>> {
    let bus_ids = input
        .lines()
        .last()
        .map(|id_list| {
            id_list
                .split_terminator(',')
                .enumerate()
                .filter_map(|(idx, id)| id.parse().map(|id| (idx, id)).ok())
                .collect::<Vec<(usize, usize)>>()
        })
        .ok_or(anyhow::anyhow!("second line must be valid bus ids"))?;

    let mut departure_time = 0;
    while departure_time < usize::MAX {
        let (all_match, matching_id_product) =
            bus_ids
                .iter()
                .fold((true, 1), |(all_match, matching_id_product), (idx, id)| {
                    if (departure_time + idx) % id == 0 {
                        (all_match && true, matching_id_product * id)
                    } else {
                        (false, matching_id_product)
                    }
                });

        if all_match {
            return Ok(Some(departure_time));
        }

        departure_time = departure_time.saturating_add(matching_id_product);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r"939
7,13,x,x,59,x,31,19";

        assert_eq!(part_one(input).unwrap(), Some(295));
        assert_eq!(part_two(input).unwrap(), Some(1068781));
    }

    #[test]
    fn real_input() {
        let input = include_str!("../data/day_13.txt");

        assert_eq!(part_one(input).unwrap(), Some(4_782));
        assert_eq!(part_two(input).unwrap(), Some(1_118_684_865_113_056));
    }
}
