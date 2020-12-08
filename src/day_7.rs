use std::collections::HashMap;

type CapacityDag<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

pub fn part_one(input: &str) -> usize {
    let dag = build_capacity_dag(input);

    let mut cache = HashMap::new();
    dag.keys()
        .filter(|node| dfs("shiny gold", node, &dag, &mut cache))
        .count()
}

pub fn part_two(input: &str) -> usize {
    let dag = build_capacity_dag(input);

    let mut cache = HashMap::new();
    subgraph_capacity("shiny gold", &dag, &mut cache)
}

fn build_capacity_dag(input: &str) -> CapacityDag<'_> {
    input.lines().fold(CapacityDag::new(), |mut dag, rules| {
        let mut rule = rules.trim_end_matches('.').splitn(2, " bags contain");
        if let (Some(subject_name), Some(rule_contents)) = (rule.next(), rule.next()) {
            let contents = rule_contents
                .split_terminator(',')
                .flat_map(|dependant_bag| {
                    let mut parts = dependant_bag.trim().splitn(2, ' ');
                    if let (Some(amount), Some(color)) = (
                        parts.next().and_then(|a| a.parse::<usize>().ok()),
                        parts.next().and_then(|color| color.rsplitn(2, ' ').last()),
                    ) {
                        Some((amount, color))
                    } else {
                        None
                    }
                })
                .collect();

            dag.insert(subject_name, contents);
        }
        dag
    })
}

fn dfs<'a>(
    target: &str,
    node: &str,
    dag: &'a CapacityDag,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&res) = cache.get(node) {
        return res;
    };

    dag.get(node).map_or(false, |edges| {
        edges.iter().any(|edge| {
            if edge.1 == target {
                true
            } else {
                let res = dfs(target, edge.1, dag, cache);
                cache.insert(edge.1, res);
                res
            }
        })
    })
}

fn subgraph_capacity<'a>(
    node: &str,
    dag: &'a CapacityDag,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&res) = cache.get(node) {
        return res;
    };

    dag.get(node)
        .map(|edges| {
            edges
                .iter()
                .map(|edge| {
                    let cap = subgraph_capacity(edge.1, dag, cache);
                    cache.insert(edge.1, cap);
                    edge.0 + (edge.0 * cap)
                })
                .sum()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_7.txt");

    #[test]
    fn examples() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(part_one(input), 4);
        assert_eq!(part_two(input), 32);

        let input_2 = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(part_two(input_2), 126);
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT), 155);
        assert_eq!(part_two(INPUT), 54_803);
    }
}
