use std::mem;

pub fn part_one(input: &str) -> Result<i32, i32> {
    let program = build_program(input);

    execute_program(&program)
}

pub fn part_two(input: &str) -> Result<i32, i32> {
    let mut program = build_program(input);

    for flip_idx in 0..program.len() {
        let inst = program[flip_idx].0;
        if ["jmp", "nop"].contains(&inst) {
            let prev_inst = if inst == "jmp" {
                mem::replace(&mut program[flip_idx].0, "nop")
            } else {
                mem::replace(&mut program[flip_idx].0, "jmp")
            };

            if let terminated_success @ Ok(_) = execute_program(&program) {
                return terminated_success;
            };

            program[flip_idx].0 = prev_inst;
        }
    }

    return Err(0);
}

fn build_program(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .flat_map(|line| {
            let mut parts = line.split_whitespace();
            if let (Some(inst), Some(arg)) =
                (parts.next(), parts.next().and_then(|num| num.parse().ok()))
            {
                Some((inst, arg))
            } else {
                None
            }
        })
        .collect()
}

fn execute_program(program: &[(&str, i32)]) -> Result<i32, i32> {
    let mut acc = 0;
    let mut pc = 0;
    let mut inst_executed = vec![false; program.len()];

    while !inst_executed[pc] {
        inst_executed[pc] = true;
        match program[pc] {
            ("acc", arg) => {
                acc += arg;
                pc += 1;
            }
            ("jmp", arg) => pc = (pc as i32 + arg) as usize,
            ("nop", _) => pc += 1,
            other => panic!("found unexpected instruction {:?}", other),
        };

        if pc == program.len() {
            return Ok(acc);
        }
    }

    Err(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../data/day_8.txt");

    #[test]
    fn examples() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(part_one(input), Err(5));
        assert_eq!(part_two(input), Ok(8));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_one(INPUT), Err(1521));
        assert_eq!(part_two(INPUT), Ok(1016));
    }
}
