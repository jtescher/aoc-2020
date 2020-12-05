const NEEDED_FIELDS: [Option<&'static str>; 7] = [
    Some("byr"),
    Some("iyr"),
    Some("eyr"),
    Some("hgt"),
    Some("hcl"),
    Some("ecl"),
    Some("pid"),
];

/// Currently permissive, could return false for duplicate or unknown fields.
pub fn part_one(input: &str) -> usize {
    input
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

pub fn part_two(input: &str) -> usize {
    input
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_4.txt");

    #[test]
    fn examples() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(part_one(input), 2);

        let invalid_input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        assert_eq!(part_two(invalid_input), 0);

        let valid_input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part_two(valid_input), 4);
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT), 170);
        assert_eq!(part_two(INPUT), 103);
    }
}
