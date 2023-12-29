use regex::Regex;
use std::env;
use std::fs;

struct Round {
    r: u32,
    g: u32,
    b: u32,
}

impl Round {
    fn new() -> Self {
        Round { r: 0, g: 0, b: 0 }
    }
}

struct Game {
    rounds: Vec<Round>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let games = read_input(filename);

    println!("part 1: {}", part1(&games));
    println!("part 2: {}", part2(&games));
}

fn read_input(filename: &str) -> Vec<Game> {
    let file = fs::read_to_string(filename).expect("Failed to read file");

    let game_re = Regex::new(r"Game \d+: (?P<rest>.*)").expect("Failed to compile regex");
    let color_re = Regex::new(r"(?P<count>\d+) (?P<color>red|green|blue)")
        .expect("Failed to compile color regex");

    file.lines()
        .map(|l| Game {
            rounds: game_re.captures(l).unwrap()["rest"]
                .split(";")
                .map(|set| {
                    let mut round = Round::new();
                    set.split(", ").for_each(|color_str| {
                        let cap = color_re.captures(color_str).unwrap();
                        let count = cap["count"].parse::<u32>().unwrap();
                        let color = &cap["color"];

                        match color {
                            "red" => round.r = count,
                            "green" => round.g = count,
                            "blue" => round.b = count,
                            _ => println!("Unknown color {}", color),
                        }
                    });
                    round
                })
                .collect(),
        })
        .collect()
}

fn part1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            let round_maxes = game
                .rounds
                .iter()
                .fold(Round::new(), |mut max_round, round| {
                    max_round.r = round.r.max(max_round.r);
                    max_round.g = round.g.max(max_round.g);
                    max_round.b = round.b.max(max_round.b);
                    max_round
                });

            round_maxes.r <= 12 && round_maxes.g <= 13 && round_maxes.b <= 14
        })
        .map(|(game_index, _)| game_index as u32 + 1)
        .sum()
}

fn part2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .enumerate()
        .map(|(_, game)| {
            let round_maxes = game
                .rounds
                .iter()
                .fold(Round::new(), |mut max_round, round| {
                    max_round.r = round.r.max(max_round.r);
                    max_round.g = round.g.max(max_round.g);
                    max_round.b = round.b.max(max_round.b);
                    max_round
                });

            round_maxes
        })
        .map(|round_maxes| round_maxes.r * round_maxes.g * round_maxes.b)
        .sum()
}
