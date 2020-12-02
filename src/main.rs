#![allow(dead_code)]

mod day_1;
mod day_2;

fn main() -> anyhow::Result<()> {
    dbg!(day_2::part_two()?);

    Ok(())
}
