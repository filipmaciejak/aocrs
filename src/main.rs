use std::io::Read;

mod year_2015;

fn main() -> Result<(), std::io::Error> {
    let mut buffer: String = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let output_1: String = year_2015::day_01::part1::solution(&buffer);
    let output_2: String = year_2015::day_01::part2::solution(&buffer);
    println!("part1: {}", output_1);
    println!("part2: {}", output_2);
    Ok(())
}
