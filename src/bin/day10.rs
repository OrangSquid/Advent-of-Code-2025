use std::collections::VecDeque;

use aoc_2025::readlines::read_lines;
use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, lp_solve, variable
};

fn part_1(number_of_lights: usize, light_final_state: BitVec<u16>, buttons: &[BitVec<u16>]) -> u64 {
    let starting_light = bitvec![u16, Lsb0; 0; number_of_lights];
    let mut lights_states = VecDeque::from([(0, starting_light)]);
    while let (pushed, light_state) = lights_states.pop_front().unwrap() {
        for press in buttons {
            let new_light_state = light_state.clone() ^ press;
            if new_light_state == light_final_state {
                return pushed + 1;
            } else {
                lights_states.push_back((pushed + 1, new_light_state));
            }
        }
    }
    0
}

fn part_2(joltage_final_state: Vec<u32>, buttons: &[BitVec<u16>]) -> u64 {
    let mut problem = ProblemVariables::new();
    let variables: Vec<_> = buttons
        .iter()
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();
    let objective: Expression = variables.iter().sum();

    let mut model = problem.minimise(objective.clone()).using(lp_solve);

    for (idx1, &joltage_section) in joltage_final_state.iter().enumerate() {
        let lol: Expression = buttons
            .iter()
            .enumerate()
            .filter_map(|(idx2, button)| {
                if *button.get(idx1).unwrap() {
                    Some(variables[idx2])
                } else {
                    None
                }
            })
            .sum();
        model = model.with(lol.eq(joltage_section));
    }
    model.solve().unwrap().eval(&objective) as u64
}

fn main() {
    let lines = read_lines("input/day10.txt");
    let (result_1, result_2) = lines
        .map(|line| {
            let mut line_it = line.split(" ");
            let lights_repr = line_it.next().unwrap();
            let number_of_lights = lights_repr.len() - 2;
            let mut lights_final_state = bitvec![u16, Lsb0; 0];
            for light_section in lights_repr.chars() {
                match light_section {
                    '[' => (),
                    ']' => (),
                    '.' => lights_final_state.push(false),
                    '#' => lights_final_state.push(true),
                    _ => panic!(),
                }
            }
            lights_final_state.remove(0);
            let buttons: Vec<BitVec<u16>> = line_it
                .clone()
                .take_while(|button| button.starts_with("("))
                .map(|button| button.trim_end_matches(")").trim_start_matches("("))
                .map(|button| {
                    let mut button_bitvec = bitvec![u16, Lsb0; 0; number_of_lights];
                    for press in button.split(",") {
                        button_bitvec.insert(press.parse().unwrap(), true);
                    }
                    button_bitvec
                })
                .collect();
            let result_1 = part_1(number_of_lights, lights_final_state, &buttons);

            let joltage: Vec<u32> = line_it
                .last()
                .unwrap()
                .split(",")
                .map(|button| button.trim_end_matches("}").trim_start_matches("{"))
                .map(|x| x.parse().unwrap())
                .collect();
            let result_2 = part_2(joltage, &buttons);

            (result_1, result_2)
        })
        .reduce(|(x1, y1), (x2, y2)| (x1 + x2, y1 + y2))
        .unwrap();

    println!("{result_1}");
    println!("{result_2}");
}
