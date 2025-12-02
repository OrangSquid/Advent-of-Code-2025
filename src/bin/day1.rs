use aoc_2025::readlines::read_lines;

const DIAL_SIZE: i16 = 100;
const START_POS: i16 = 50;

fn parsing(next_move: String) -> i16 {
    let left_or_right = next_move.get(..1).unwrap();
    let ticks = next_move.get(1..).unwrap().parse::<i16>().unwrap();
    match left_or_right {
        "R" => ticks,
        "L" => -ticks,
        _ => panic!(),
    }
}

fn part1_2(lines: impl Iterator<Item = String>) -> (i16, i16, i16) {
    lines.fold((0, 0, START_POS), |acc, next_move| {
        let ticks = parsing(next_move);
        let (landed, passed, position) = acc;
        let new_position = position + ticks;
        let passed = if new_position <= 0 {
            passed + (position != 0) as i16 - (new_position / DIAL_SIZE)
        } else {
            passed + new_position / DIAL_SIZE
        };
        (landed + (new_position == 0) as i16, passed, new_position.rem_euclid(DIAL_SIZE))
    })
}

fn main() {
    let lines = read_lines("input/day1.txt");

    let (result1, result2, _) = part1_2(lines);
    println!("{}", result1);
    println!("{}", result2);
}
