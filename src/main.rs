use std::io::Read;

mod year_2015;

fn main() -> Result<(), std::io::Error> {
    let mut buffer: String = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let output: String = year_2015::day_01::part1::solution(&buffer);
    println!("{}", output);
    Ok(())
}
