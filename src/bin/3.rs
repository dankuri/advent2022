// WARN: for this to work use nightly build - i've used 1.68.0-nightly (dfe3fe710 2022-12-09)
#![feature(iter_array_chunks)]
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("src/inputs/3.txt").unwrap();
    let mut alphabet: Vec<char> = ('a'..='z').into_iter().collect();
    let mut alphabet_uppercase: Vec<char> = ('A'..='Z').into_iter().collect();
    alphabet.append(&mut alphabet_uppercase);

    // INFO: part 1:
    // dis my stinky nooby way
    let mut sum = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        for char in left.chars() {
            if right.contains(char) {
                sum += alphabet.iter().position(|&x| x == char).unwrap() + 1;
                break;
            }
        }
    }
    println!("{sum}");

    // INFO: part 2
    // dis is theprimeagen version
    const START_LOWER: u8 = b'a' - 1;
    const START_UPPER: u8 = b'A' - 1;
    let result: u32 = input
        .lines()
        .array_chunks::<3>()
        .flat_map(|group| {
            return group
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<_>>().into_iter())
                .fold(HashMap::new(), |mut map: HashMap<char, u32>, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 3);
        })
        .map(|c| c.0)
        .map(|c| {
            let value = if c.is_ascii_lowercase() {
                c as u8 - START_LOWER
            } else {
                c as u8 - START_UPPER + 26
            } as u32;

            return value;
        })
        .sum::<u32>();

    println!("{result}")
}
