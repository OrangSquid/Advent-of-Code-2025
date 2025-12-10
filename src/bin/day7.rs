use aoc_2025::readlines::read_lines;

fn main() {
    let mut lines = read_lines("input/day7.txt");
    let first_line = lines.next().unwrap();

    let number_columns = first_line.len();

    let first_beam: Vec<_> = first_line
        .char_indices()
        .filter(|(_, cell)| *cell == 'S')
        .map(|(y, _)| (0 as usize, y))
        .collect();

    let (_, s_index) = first_beam.first().unwrap();

    let quantum_beams: Vec<usize> = (0..number_columns)
        .map(|x| if x == *s_index { 1 } else { 0 })
        .collect();

    let (result_1, _, result_2) = lines.enumerate().fold(
        (0, first_beam, quantum_beams),
        |(splitted, acc_beams, quantum_beams), (_, line)| {
            let char_vec: Vec<char> = line.chars().collect();
            let mut new_quantum_beams: Vec<usize> = (0..quantum_beams.len()).map(|_| 0).collect();
            let mut next_beams: Vec<_> = acc_beams
                .iter()
                .flat_map(|(x, y)| {
                    if char_vec[*y] == '^' {
                        new_quantum_beams[*y + 1] += quantum_beams[*y];
                        new_quantum_beams[*y - 1] += quantum_beams[*y];
                        vec![(x + 1, y - 1), (x + 1, y + 1)]
                    } else {
                        new_quantum_beams[*y] += quantum_beams[*y];
                        vec![(x + 1, *y)]
                    }
                })
                .collect();
            let splited_beams = splitted + next_beams.len() - acc_beams.len();
            next_beams.dedup();
            (splited_beams, next_beams, new_quantum_beams)
        },
    );

    let result_2: usize = result_2.iter().sum();

    println!("{result_1}");
    println!("{result_2}");
}
