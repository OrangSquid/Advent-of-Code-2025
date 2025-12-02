use aoc_2025::readlines::read_lines;

fn find_divisors(x: usize) -> impl Iterator<Item = usize> {
    (2..(x + 1)).filter(move |y| x % y == 0)
}

fn part2(string_part: &str, entire_string: &str, number_of_times: usize) -> bool {
    let string_part_repeated = string_part.repeat(number_of_times);
    entire_string == string_part_repeated
}

fn part1_2(start: i64, end: i64) -> (i64, i64) {
    (start..(end + 1)).fold((0, 0), |(acc1, acc2), value| {
        let converted_string = value.to_string();
        let size = converted_string.len();
        let result1 = if size % 2 != 0 {
            acc1
        } else {
            let part1 = converted_string.get(..size / 2).unwrap();
            let part2 = converted_string.get(size / 2..).unwrap();
            if part1 == part2 { acc1 + value } else { acc1 }
        };

        let result2 = find_divisors(size).any(|x| {
            part2(
                converted_string.get(..size / x).unwrap(),
                &converted_string,
                x,
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
        let begin: i64 = splited.next().unwrap().parse().unwrap();
        let end: i64 = splited.next().unwrap().parse().unwrap();
        let (part1, part2) = part1_2(begin, end);
        (acc1 + part1, acc2 + part2)
    });
    println!("{}", result1);
    println!("{}", result2);
}
