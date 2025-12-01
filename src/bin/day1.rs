use aoc_2025::readlines::read_lines;

const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

fn parsing(next_move: &str) -> i32 {
    let left_or_right = next_move.get(..1).unwrap();
    let ticks = next_move.get(1..).unwrap().parse::<i32>().unwrap();
    match left_or_right {
        "R" => ticks,
        "L" => -ticks,
        _ => panic!(),
    }
}

fn part1(lines: &[String]) -> (i32, i32) {
    lines.iter().fold((0, START_POS), |acc, next_move| {
        let ticks = parsing(&next_move);
        let (landed, position) = acc;
        let new_position = (position + ticks).rem_euclid(DIAL_SIZE);
        (landed + (new_position == 0) as i32, new_position)
    })
}

fn part2(lines: &[String]) -> (i32, i32) {
    lines.iter().fold((0, START_POS), |acc, next_move| {
        let ticks = parsing(&next_move);
        let (landed, position) = acc;
        let new_position = position + ticks;
        let landed = if new_position <= 0 {
            landed + (position != 0) as i32 - (new_position / DIAL_SIZE)
        } else {
            landed + new_position / DIAL_SIZE
        };
        (landed, new_position.rem_euclid(DIAL_SIZE))
    })
}

fn main() {
    let lines = read_lines("input/day1.txt");

    let (result1, _) = part1(&lines);
    println!("{}", result1);
    let (result2, _) = part2(&lines);
    println!("{}", result2);
}
