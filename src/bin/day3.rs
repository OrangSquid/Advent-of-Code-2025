use aoc_2025::readlines::read_lines;

fn part1(bank: &[u64], multiplier: u64, digits: usize, acc: u64) -> u64 {
    let size = bank.len();
    let (idx, value) = bank[..size - digits]
        .iter()
        .enumerate()
        .max_by(|(idx_a, val_a), (idx_b, val_b)| {
            val_a.cmp(val_b).then(idx_b.cmp(idx_a)) 
        })
        .unwrap();
    if multiplier > 1 {
        part1(&bank[idx+1..], multiplier / 10, digits - 1, acc + (multiplier * value))
    }
    else {
        acc + *value
    }
}

fn main() {
    let lines = read_lines("input/day3.txt");
    let result_1: u64 = lines
        .map(|line| {
            let bank: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u64))
                .collect();
            
            part1(&bank, 100000000000, 12, 0)
        })
        .sum();
    
    println!("{result_1}");
}
