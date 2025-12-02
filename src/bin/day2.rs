use aoc_2025::readlines::read_lines;

fn find_divisors(x: usize) -> impl Iterator<Item = usize> {
    (1..x).filter(move |&y| x % y == 0)
}

fn part2(number: u64, divisor: u64, repeats: u32, digits_in_number: u32) -> bool {
    let lol = number / divisor;
    let repeated = (0..repeats).into_iter().fold(0, |acc, x| {
        let lol2 = 10_u64.pow(x * digits_in_number);
        acc + lol * lol2
    });
    number == repeated
}

fn part1_2(start: u64, end: u64) -> (u64, u64) {
    (start..=end).fold((0, 0), |(acc1, acc2), value| {
        let size = (value.ilog10() + 1) as usize;
        let result1 = if size % 2 != 0 {
            acc1
        } else {
            let divisor = 10_u64.pow((size / 2) as u32);
            let part1 = value / divisor;
            let part2 = value % divisor;
            if part1 == part2 { acc1 + value } else { acc1 }
        };

        let result2 = find_divisors(size).any(|x| {
            let divisor = 10_u64.pow((size - x) as u32);
            let repeats = size / x;
            part2(
                value,
                divisor,
                repeats as u32,
                x as u32
            ) 
        });
        (result1, if result2 { acc2 + value } else { acc2 })
    })
}

fn main() {
    let binding = read_lines("input/day2.txt").next().unwrap();
    let line = binding.split(",");

    let (result1, result2) = line.fold((0, 0), |(acc1, acc2), range| {
        let mut splited = range.split("-");
        let begin: u64 = splited.next().unwrap().parse().unwrap();
        let end: u64 = splited.next().unwrap().parse().unwrap();
        let (part1, part2) = part1_2(begin, end);
        (acc1 + part1, acc2 + part2)
    });
    println!("{}", result1);
    println!("{}", result2);
}
