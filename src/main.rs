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

fn map_solution(problem: ProblemID) -> impl Fn(&str) -> String {
    match problem {
        ProblemID { year: 2015, day: 1, part: 1 } => year_2015::day_01::part1::solution,
        ProblemID { year: 2015, day: 1, part: 2 } => year_2015::day_01::part2::solution,
        ProblemID { year: 2015, day: 2, part: 1 } => year_2015::day_02::part1::solution,
        ProblemID { year: 2015, day: 2, part: 2 } => year_2015::day_02::part2::solution,
        _ => panic!()
    }
}

#[derive(Clone, Copy)]
struct ProblemID {
    year: i32,
    day: i32,
    part: i32
}

fn parse_problem_identifier(identifier: String) -> ProblemID {
    let [year, day, part]: [i32; 3] = identifier
        .split('/')
        .map(|x| {
            str::parse(x).unwrap_or_default()
        })
        .collect::<Vec<i32>>()
        .try_into().unwrap();
    ProblemID { year, day, part }
}

fn main() -> Result<(), std::io::Error> {

    let args = Args::parse();

    let problem = parse_problem_identifier(args.problem);

    let solution = map_solution(problem);

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
                    year = problem.year,
                    day = problem.day
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
