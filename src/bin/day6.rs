use aoc_2025::readlines::read_lines_to_vec;

fn part1(lines: &[String]) -> u64 {
    let numbers: Vec<Vec<u64>> = lines[..(lines.len() - 1)]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let operations = lines
        .last()
        .unwrap();
    
    operations
        .split_whitespace()
        .enumerate()
        .map(|(idx, x)| {
            let column_it = numbers.iter().map(|y| y[idx]);
            let lol: u64 = match x {
                "+" => column_it.sum(),
                "*" => column_it.product(),
                _ => panic!("WHAT????"),
            };
            lol
        })
        .sum()
}

fn vec_char_to_u64(column: &[char]) -> u64 {
    column.iter().collect::<String>().parse().unwrap()
}

fn part2(lines: Vec<Vec<char>>) -> u64 {
    let number_columns = lines[0].len();
    let mut numbers: Vec<Vec<u64>> = Vec::with_capacity(number_columns);
    let mut operation_numbers: Vec<u64> = Vec::new();
    let mut column = Vec::with_capacity(lines.len());
    for i in 0..number_columns {
        let mut found_digit = false;
        column.clear();
        for j in 0..lines.len() - 1 {
            let cell = lines[j][i];
            if !cell.is_whitespace() {
                column.push(cell);
                found_digit = true;
            }
        }
        if !found_digit {
            numbers.push(operation_numbers);
            operation_numbers = Vec::new();
        } else {
            operation_numbers.push(vec_char_to_u64(&column));
        }
    }
    numbers.push(operation_numbers);

    let operations: String = lines
        .last()
        .unwrap()
        .iter()
        .collect();

    operations
        .split_whitespace()
        .enumerate()
        .map(|(idx, x)| {
            let column_it = numbers[idx].iter();
            let lol: u64 = match x {
                "+" => column_it.sum(),
                "*" => column_it.product(),
                _ => panic!("WHAT????"),
            };
            lol
        })
        .sum()
}

fn main() {
    let lines = read_lines_to_vec("input/day6.txt");
    let result_1 = part1(&lines);
    let lines_as_chars: Vec<Vec<char>> = lines.iter().map(move |x| x.chars().collect()).collect();
    let result_2 = part2(lines_as_chars);

    println!("{result_1}");
    println!("{result_2}");
}
