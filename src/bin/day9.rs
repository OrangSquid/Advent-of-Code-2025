use std::{
    collections::{BinaryHeap, HashSet},
    usize,
};

use aoc_2025::readlines::read_lines;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn rectangle_area<'a>(&'a self, other: &'a Point) -> Area<'a> {
        let area = (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1);
        Area {
            point1: &self,
            point2: other,
            area,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Area<'a> {
    area: usize,
    point1: &'a Point,
    point2: &'a Point,
}

impl<'a> Area<'a> {
    fn get_borders(&self) -> HashSet<(usize, usize)> {
        let x_range = self.point1.x.min(self.point2.x)..=self.point1.x.max(self.point2.x);
        let y_range = self.point1.y.min(self.point2.y)..=self.point1.y.max(self.point2.y);

        // Nested flat_map creates the grid (Cartesian product)
        x_range
            .flat_map(move |x| {
                // We must clone y_range because the iterator is consumed for every x
                y_range.clone().map(move |y| (x, y))
            })
            .collect()
    }
}

fn inside_valid_area(borders: &HashSet<(usize, usize)>, area: &Area) -> bool {
    let y_range = area.point1.y.min(area.point2.y)..=area.point1.y.max(area.point2.y);

    let mut border_state = false;
    let mut previous_was_border = false;

    for y in y_range.clone() {
        for x in 0..100000 {
            if borders.contains(&(x, y)) {
                if !previous_was_border {
                    border_state = !border_state;
                    previous_was_border = true;
                }
            } else {
                previous_was_border = false;
            }
            if rectangle_borders.contains(&(x,y)) && !border_state {
                return false;
            }
        }
    }

    true
}

fn main() {
    let points: Vec<Point> = read_lines("input/day9.txt")
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let borders: HashSet<(usize, usize)> = points
        .iter()
        .zip(points.iter().cycle().skip(1)) // Pair item i with i+1 (wrapping)
        .take(points.len())
        .flat_map(|(p1, p2)| {
            let x_range = p1.x.min(p2.x)..=p1.x.max(p2.x);
            let y_range = p1.y.min(p2.y)..=p1.y.max(p2.y);

            // Nested flat_map creates the grid (Cartesian product)
            x_range.flat_map(move |x| {
                // We must clone y_range because the iterator is consumed for every x
                y_range.clone().map(move |y| (x, y))
            })
        })
        .collect();

    let mut max_area: BinaryHeap<Area> = points[..(points.len() - 1)]
        .iter()
        .enumerate()
        .flat_map(|(idx, point1)| {
            points[(idx + 1)..]
                .iter()
                .map(|point2| point1.rectangle_area(point2))
        })
        .collect();


    let mut result = max_area.pop().unwrap();
    println!("{}", result.area);

    while let result2 = max_area.peek().unwrap()
        && !inside_valid_area(&borders, result2)
    {
        max_area.pop();
    }

    result = max_area.pop().unwrap();
    println!("{}", result.area);
}
