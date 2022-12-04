use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/inputs/2.txt").unwrap();

    // * part 1

    let mut table1: HashMap<&str, u32> = HashMap::new();
    let mut sum1: u32 = 0;

    // * Rock (1) - A & X, Paper (2) - B & Y, Scissors (3) - C & Z
    // * Win - 6, Draw - 3, Lose - 0
    table1.insert("A X", 1 + 3);
    table1.insert("A Y", 2 + 6);
    table1.insert("A Z", 3 + 0);
    table1.insert("B X", 1 + 0);
    table1.insert("B Y", 2 + 3);
    table1.insert("B Z", 3 + 6);
    table1.insert("C X", 1 + 6);
    table1.insert("C Y", 2 + 0);
    table1.insert("C Z", 3 + 3);

    for line in input.lines() {
        sum1 += table1.get(line).unwrap();
    }

    println!("Part 1: {sum1}");

    // * part 2

    let mut table2: HashMap<&str, u32> = HashMap::new();
    let mut sum2: u32 = 0;

    // * Rock (1) - A, Paper (2) - B, Scissors (3) - C
    // * Win (6) - Z, Draw (3) - Y, Lose (0) - X
    table2.insert("A X", 3 + 0);
    table2.insert("A Y", 1 + 3);
    table2.insert("A Z", 2 + 6);
    table2.insert("B X", 1 + 0);
    table2.insert("B Y", 2 + 3);
    table2.insert("B Z", 3 + 6);
    table2.insert("C X", 2 + 0);
    table2.insert("C Y", 3 + 3);
    table2.insert("C Z", 1 + 6);

    for line in input.lines() {
        sum2 += table2.get(line).unwrap();
    }

    println!("Part 2: {sum2}");
}
