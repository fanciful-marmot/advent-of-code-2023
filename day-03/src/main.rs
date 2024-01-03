use core::ops::Range;
use regex::Regex;
use std::env;
use std::fs;

struct Part {
    id: u32,
    range: Range<usize>,
}

struct Symbol {
    sym_type: char,
    index: usize,
}

struct Row {
    parts: Vec<Part>,
    symbol_indices: Vec<Symbol>,
}

struct Engine {
    rows: Vec<Row>,
}

impl Engine {
    fn get_part_number(&self, r: usize, index: usize) -> Option<u32> {
        let row = self.rows.get(r)?;

        let part = row.parts.iter().find(|&part| part.range.contains(&index))?;

        Some(part.id)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let engine = read_input(filename);

    println!("part 1: {}", part1(&engine));
    println!("part 2: {}", part2(&engine));
}

fn read_input(filename: &str) -> Engine {
    let file = fs::read_to_string(filename).expect("Failed to read file");

    let part_re = Regex::new(r"(?P<part>\d+)").expect("Failed to compile regex");

    let rows = file
        .lines()
        .map(|l| {
            let parts = part_re
                .find_iter(l)
                .map(|part| Part {
                    id: part.as_str().parse::<u32>().unwrap(),
                    range: part.range(),
                })
                .collect();

            let indices = l
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
                .map(|(index, c)| Symbol {
                    sym_type: c,
                    index: index,
                })
                .collect();

            Row {
                parts: parts,
                symbol_indices: indices,
            }
        })
        .collect();

    Engine { rows: rows }
}

fn part1(engine: &Engine) -> u32 {
    engine
        .rows
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.symbol_indices
                .iter()
                .map(|symbol| {
                    let mut part_ids = Vec::with_capacity(9);
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let next_row = i + r as i32;
                            let next_index = j + symbol.index as i32;
                            if next_row >= 0 && next_index >= 0 {
                                let x = engine
                                    .get_part_number(next_row as usize, next_index as usize)
                                    .unwrap_or_default();

                                part_ids.push(x);
                            }
                        }
                    }

                    // Assumes part ids are unique
                    part_ids.dedup();
                    part_ids.iter().sum::<u32>()
                })
                .sum::<u32>()
        })
        .sum()
}

fn part2(engine: &Engine) -> u32 {
    engine
        .rows
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.symbol_indices
                .iter()
                .filter(|&symbol| symbol.sym_type == '*') // Only check gears
                .map(|symbol| {
                    let mut part_ids = Vec::with_capacity(9);
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let next_row = i + r as i32;
                            let next_index = j + symbol.index as i32;
                            if next_row >= 0 && next_index >= 0 {
                                let x = engine
                                    .get_part_number(next_row as usize, next_index as usize)
                                    .unwrap_or_default();

                                if x > 0 {
                                    part_ids.push(x);
                                }
                            }
                        }
                    }

                    // Assumes part ids are unique
                    part_ids.dedup();

                    // Gears are adjacent to exactly 2 parts
                    if part_ids.len() == 2 {
                        part_ids.iter().product::<u32>()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}
