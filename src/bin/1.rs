use std::fs;

fn main() {
    let input = fs::read_to_string("src/inputs/1.txt").unwrap();
    let mut max: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    max.sort_by(|a, b| b.cmp(a));

    let mut sum_top3: u32 = 0;
    for n in 0..=2 {
        sum_top3 += max[n];
    }

    println!("Top elf carries: {:?}", max[0]);
    println!("Top 3 elves carry: {:?}", sum_top3);
}
