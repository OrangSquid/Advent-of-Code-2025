use std::result;

use aoc_2025::readlines::read_lines_to_vec_char;

fn part1(lines: &[Vec<char>], col: usize, line: usize) -> bool {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let line_to_check = line as isize + i;
            if line_to_check < 0 || line_to_check >= lines.len() as isize {
                continue;
            }
            let col_to_check = col as isize + j;
            if col_to_check < 0 || col_to_check >= lines.len() as isize {
                continue;
            }
            count += (lines[line_to_check as usize][col_to_check as usize] == '@') as i32;
        }
    }
    count < 4
}

fn get_iterator_of_paper(lines: &[Vec<char>]) -> impl Iterator<Item = (usize, usize)> {
    lines.iter().enumerate().flat_map(|(line_n, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, x)| **x == '@')
            .map(move |(col_n, _)| (col_n, line_n))
    })
}

fn main() {
    let mut lines = read_lines_to_vec_char("input/day4.txt");

    let mut removeable: Vec<_> = get_iterator_of_paper(&lines)
        .filter(|(x, y)| part1(&lines, *x, *y))
        .collect();
    let result1 = removeable.len();
    let mut result2 = result1;

    while removeable.len() > 0 {
        for (x, y) in removeable {
            lines[y][x] = '.';
        }
        removeable = get_iterator_of_paper(&lines)
            .filter(|(x, y)| part1(&lines, *x, *y))
            .collect();
        result2 += removeable.len();
    }

    println!("{result1}");
    println!("{result2}");
}
