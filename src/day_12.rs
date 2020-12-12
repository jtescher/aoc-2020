pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let instructions = parse_instructions(input)?;

    let (_, x, y) = instructions.into_iter().fold(
        (Direction::East, 0, 0),
        |(s_dir, sx, sy), inst| match inst {
            Instruction::Move { move_dir, amount } => match move_dir {
                Direction::North => (s_dir, sx, sy + amount),
                Direction::South => (s_dir, sx, sy - amount),
                Direction::East => (s_dir, sx + amount, sy),
                Direction::West => (s_dir, sx - amount, sy),
            },
            Instruction::Turn { amount } => {
                let new_s_dir = (((s_dir as isize + (amount / 90)) % 4) as u8).into();
                (new_s_dir, sx, sy)
            }
            Instruction::Forward { amount } => match s_dir {
                Direction::North => (s_dir, sx, sy + amount),
                Direction::South => (s_dir, sx, sy - amount),
                Direction::East => (s_dir, sx + amount, sy),
                Direction::West => (s_dir, sx - amount, sy),
            },
        },
    );

    Ok((x.abs() + y.abs()) as usize)
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let instructions = parse_instructions(input)?;

    let ((x, y), _) =
        instructions
            .into_iter()
            .fold(((0, 0), (10, 1)), |((sx, sy), (wx, wy)), inst| match inst {
                Instruction::Move { move_dir, amount } => match move_dir {
                    Direction::North => ((sx, sy), (wx, wy + amount)),
                    Direction::South => ((sx, sy), (wx, wy - amount)),
                    Direction::East => ((sx, sy), (wx + amount, wy)),
                    Direction::West => ((sx, sy), (wx - amount, wy)),
                },
                Instruction::Turn { amount } => match amount {
                    90 => ((sx, sy), (wy, -wx)),
                    180 => ((sx, sy), (-wx, -wy)),
                    270 => ((sx, sy), (-wy, wx)),
                    other => panic!("expected increment of 90 degrees, found {}", other),
                },
                Instruction::Forward { amount } => ((sx + amount * wx, sy + amount * wy), (wx, wy)),
            });

    Ok((x.abs() + y.abs()) as usize)
}

fn parse_instructions(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| match line.split_at(1) {
            ("N", arg) => Ok(Instruction::Move {
                move_dir: Direction::North,
                amount: arg.parse()?,
            }),
            ("S", arg) => Ok(Instruction::Move {
                move_dir: Direction::South,
                amount: arg.parse()?,
            }),
            ("E", arg) => Ok(Instruction::Move {
                move_dir: Direction::East,
                amount: arg.parse()?,
            }),
            ("W", arg) => Ok(Instruction::Move {
                move_dir: Direction::West,
                amount: arg.parse()?,
            }),
            ("L", arg) => Ok(Instruction::Turn {
                amount: arg.parse::<isize>().map(|n| 360 - n)?,
            }),
            ("R", arg) => Ok(Instruction::Turn {
                amount: arg.parse()?,
            }),
            ("F", arg) => Ok(Instruction::Forward {
                amount: arg.parse()?,
            }),
            other => anyhow::bail!("expected instruction, got {:?}", other),
        })
        .collect()
}

enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl From<u8> for Direction {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            other => panic!("cannot convert {} to Direction", other),
        }
    }
}

enum Instruction {
    Move { move_dir: Direction, amount: isize },
    Turn { amount: isize },
    Forward { amount: isize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r"F10
N3
F7
R90
F11";

        assert_eq!(part_one(input).unwrap(), 25);
        assert_eq!(part_two(input).unwrap(), 286);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../data/day_12.txt");

        assert_eq!(part_one(input).unwrap(), 1_687);
        assert_eq!(part_two(input).unwrap(), 20_873);
    }
}
