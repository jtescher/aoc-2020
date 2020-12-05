const INPUT: &'static str = include_str!("../data/day_4.txt");
const NEEDED_FIELDS: [Option<&'static str>; 7] = [
    Some("byr"),
    Some("iyr"),
    Some("eyr"),
    Some("hgt"),
    Some("hcl"),
    Some("ecl"),
    Some("pid"),
];

// Currently permissive, could return false for duplicate or unknown fields.
pub fn part_one() -> usize {
    INPUT
        .split_terminator("\n\n")
        .filter(|line| {
            let mut needed_fields = NEEDED_FIELDS;

            for kv in line.split_whitespace() {
                let mut split = kv.split_terminator(':');
                if let (Some(key), Some(_value)) = (split.next(), split.next()) {
                    if let Some(req) = needed_fields.iter_mut().find(|req| **req == Some(key)) {
                        *req = None;
                    }
                }
            }

            needed_fields.iter().all(|req| req.is_none())
        })
        .count()
}

pub fn part_two() -> usize {
    INPUT
        .split_terminator("\n\n")
        .filter(|line| {
            let mut needed_fields = NEEDED_FIELDS;

            for kv in line.split_whitespace() {
                let mut split = kv.split_terminator(':');
                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    if let Some(req) = needed_fields.iter_mut().find(|req| **req == Some(key)) {
                        if match key {
                            "byr" => (1920..=2002).contains(&value.parse().unwrap_or(0)),
                            "iyr" => (2010..=2020).contains(&value.parse().unwrap_or(0)),
                            "eyr" => (2020..=2030).contains(&value.parse().unwrap_or(0)),
                            "hgt" => match value.split_at(value.len().saturating_sub(2)) {
                                (c, "cm") => (150..=193).contains(&c.parse().unwrap_or(0)),
                                (i, "in") => (59..=76).contains(&i.parse().unwrap_or(0)),
                                _ => false,
                            },
                            "hcl" if value.len() == 7 && value.chars().nth(0) == Some('#') => {
                                value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
                            }
                            "ecl" => {
                                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
                            }
                            "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
                            _ => false,
                        } {
                            *req = None;
                        }
                    }
                }
            }

            needed_fields.iter().all(|req| req.is_none())
        })
        .count()
}
