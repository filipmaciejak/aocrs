use clap::Parser;
use std::io::Read;

mod year_2015;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {

    #[arg(short, long)]
    problem: String,

    #[arg(short, long)]
    aoc_session: Option<String>

}

fn map_solution(year: i32, day: i32, part: i32) -> impl Fn(&str) -> String {
    match (year, day, part) {
        (2015, 1, 1) => year_2015::day_01::part1::solution,
        (2015, 1, 2) => year_2015::day_01::part2::solution,
        (2015, 2, 1) => year_2015::day_02::part1::solution,
        (2015, 2, 2) => year_2015::day_02::part2::solution,
        (_, _, _) => panic!()
    }
}

fn main() -> Result<(), std::io::Error> {

    let args = Args::parse();

    let [year, day, part]: [i32; 3] = args.problem
        .split('/')
        .map(|x| -> i32 {
            str::parse(x).unwrap_or_default()
        })
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap_or_default();

    let solution = map_solution(year, day, part);

    let mut buffer: String = String::new();

    match args.aoc_session {
        None => {
            std::io::stdin()
            .read_to_string(&mut buffer)?;
        },
        Some(cookie) => {
            buffer = {
                let client = reqwest::blocking::Client::new();
                client.request(reqwest::Method::GET, format!(
                    "https://adventofcode.com/{year}/day/{day}/input",
                ))
                .header("Cookie", format!("session={}", cookie))
                .send().unwrap()
                .text().unwrap()
            };
        }
    }

    let output: String = solution(&buffer);
    println!("output: {}", output);

    Ok(())
}
