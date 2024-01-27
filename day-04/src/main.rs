use std::collections::HashSet;
use std::env;
use std::fs;

struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let cards = read_input(filename);

    println!("part 1: {}", part1(&cards));
    println!("part 2: {}", part2(&cards));
}

fn read_input(filename: &str) -> Vec<Card> {
    let file = fs::read_to_string(filename).expect("Failed to read file");

    file.lines()
        .map(|l| {
            let mut lists_iter = l.split(':').skip(1).next().unwrap().split('|');

            let winning_numbers = lists_iter
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect();

            let numbers = lists_iter
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            Card {
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

fn part1(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card| {
            let winning_set = HashSet::<u32>::from_iter(card.winning_numbers.iter().copied());

            card.numbers.iter().fold(0, |acc, num| {
                if winning_set.contains(num) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part2(cards: &Vec<Card>) -> u32 {
    let mut init = Vec::<u32>::with_capacity(cards.len());
    init.resize(cards.len(), 1);

    cards
        .iter()
        .enumerate()
        .fold(init, |mut card_counts, (card_number, card)| {
            let winning_set = HashSet::<u32>::from_iter(card.winning_numbers.iter().copied());

            let num_wins = card.numbers.iter().fold(0, |acc, num| {
                if winning_set.contains(num) {
                    acc + 1
                } else {
                    acc
                }
            });

            for num in 1..=num_wins {
                // Increment the winning card IDs
                card_counts[num + card_number] += card_counts[card_number];
            }

            card_counts
        })
        .iter()
        .sum()
}
