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
    for n in max.iter().take(2 + 1) {
        sum_top3 += n;
    }

    println!("Part 1: {:?}", max[0]);
    println!("Part 2: {sum_top3:?}");
}
