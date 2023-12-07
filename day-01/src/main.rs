use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let input = read_input(filename);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}

fn part1(contents: &String) -> u32 {
    contents
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .map(|numbers: Vec<u32>| {
            numbers.iter().next().unwrap() * 10 + numbers.iter().last().unwrap()
        })
        .sum()
}

fn part2(contents: &String) -> u32 {
    let first_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let last_re = Regex::new(r"(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d)").unwrap();

    let string_number_lookup: Vec<String> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    contents
        .lines()
        .map(|line| {
            let first = first_re.find(line).unwrap().as_str();

            // This bizarre construct is to get around an edge case:
            // 1zeighthree
            // has last digit 'three', but using the normal regex matches 'eight'.
            // So reverse the string so we find the actual last one first
            let reversed_line = line.chars().rev().collect::<String>();
            let last_string = last_re
                .find(&reversed_line)
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>();
            let last = last_string.as_str();

            let convert = |element: &str| -> u32 {
                if element.len() == 1 {
                    // We're a numeric digit
                    element.parse().unwrap()
                } else {
                    1 + string_number_lookup
                        .iter()
                        .position(|r| r.eq_ignore_ascii_case(element))
                        .unwrap() as u32
                }
            };

            convert(first) * 10 + convert(last)
        })
        .sum()
}
