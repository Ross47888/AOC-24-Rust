#![allow(unused)]
mod complex;
use crate::complex::*;

use std::collections::HashSet;

fn main() {
    let inpf = include_str!("inputs/06ex.txt");
    println!("Part One: {}", part_one(&inpf));
    part_two(inpf);
}

fn part_two(inpf: &str) {
    let (mut pos, mut grid): (Complex, Vec<Complex>) = (Complex::new(0, 0), vec![]);
    for (line_nu, line) in inpf.lines().enumerate() {
        grid.extend(
            line.chars()
                .enumerate()
                .filter(|(i, ch)| *ch == '#')
                .map(|(x, _)| Complex::new(x as i64, line_nu as i64))
                .collect::<Vec<Complex>>(),
        );
        if let Some(x_pos) = line.chars().position(|ch| ch == '^') {
            pos = Complex::new(x_pos as i64, line_nu as i64);
        }
    }
    grid.sort_by_key(|z| (z.im, z.re));
    println!("Pos: {:?}", pos);
    for i in &grid {
        println!("{:?}", i);
    }
    let travel_lines: Vec<Vec<Complex>> = travel_2(pos, &grid);
}

fn travel_2(pos: Complex, grid: &Vec<Complex>) -> Vec<Vec<Complex>> {
    // Vec 0 = North, Vec 1 = East, Vec 2 = South, Vec 3 = West
    let mut travel_lines : Vec<Vec<HashSet<Complex>>> = vec![];
    let (mut dir, mut order) = (Complex::new(1, 0), 0);
    for i in (0..4) {
        match dir.direction() {
            Direction::North => {
                travel_lines.nth(0).push_back(complex_line(grid, pos, dir.direction()));
            },
            Direction::East =>  {
                println!("East");
            }
            Direction::South => {
                println!("South");
            }
            Direction::West => {
                println!("West");
            }
        }
        println!("{:?}", dir);
        dir *= Complex::new(0, 1);
        order += 1;
    }
    unimplemented!();
}

fn complex_line(grid: &Vec<Complex>, pos: Complex, dir: Direction) -> HashSet<Complex> {
    match dir {
            Direction::North => {
            }
            Direction::East =>  {
                println!("East");
            }
            Direction::South => {
                println!("South");
            }
            Direction::West => {
                println!("West");
            }
    }
    HashSet::<Complex>::new().insert(Complex::new(1,1))
}

fn part_one(inpf: &str) -> usize {
    let mut grid = inpf
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(move |&(x, ch)| ch == '^' || ch == '#')
                .map(move |(x, c)| (y, x, c))
        })
        .collect::<Vec<(usize, usize, char)>>();
    grid.sort();
    grid.sort_by_key(|(_, _, ch)| *ch);
    println!("{:?}", grid);
    let (x_len, y_len) = (inpf.lines().next().unwrap().len(), inpf.lines().count());
    let pos = grid.pop().unwrap();
    let (mut pos, mut dir, mut sum, mut visited): (
        (usize, usize),
        u8,
        usize,
        HashSet<(usize, usize)>,
    ) = ((pos.0, pos.1), 0, 0, HashSet::new());
    loop {
        if travel(&mut pos, &grid, &mut visited, dir, y_len, x_len) {
            return visited.len();
        }
        dir = (dir + 1) % 4;
    }
}

fn travel(
    pos: &mut (usize, usize),
    grid: &Vec<(usize, usize, char)>,
    visited: &mut HashSet<(usize, usize)>,
    dir: u8,
    y_len: usize,
    x_len: usize,
) -> bool {
    match dir {
        0 => {
            if let Some(blocker) = grid
                .iter()
                .filter(|(y, x, _)| (*x == pos.1) && (*y < pos.0))
                .last()
            {
                (blocker.0 + 1..=pos.0)
                    .map(|y| (y, pos.1))
                    .for_each(|coord| {
                        visited.insert(coord);
                    });
                *pos = (blocker.0 + 1, blocker.1);
                return false;
            }
            (0..=pos.0).map(|y| (y, pos.1)).for_each(|coord| {
                visited.insert(coord);
            });
            true
        }
        1 => {
            if let Some(blocker) = grid
                .iter()
                .filter(|(y, x, _)| (*y == pos.0) && (*x > pos.1))
                .nth(0)
            {
                (pos.1..blocker.1).map(|x| (pos.0, x)).for_each(|coord| {
                    visited.insert(coord);
                });
                *pos = (blocker.0, blocker.1 - 1);
                return false;
            }
            (pos.1..x_len).map(|x| (pos.0, x)).for_each(|coord| {
                visited.insert(coord);
            });
            true
        }
        2 => {
            if let Some(blocker) = grid
                .iter()
                .filter(|(y, x, _)| (*x == pos.1) && (*y > pos.0))
                .nth(0)
            {
                (pos.0..blocker.0).map(|y| (y, pos.1)).for_each(|coord| {
                    visited.insert(coord);
                });
                *pos = (blocker.0 - 1, blocker.1);
                return false;
            }
            (pos.0..y_len).map(|y| (y, pos.1)).for_each(|coord| {
                visited.insert(coord);
            });
            true
        }
        3 => {
            if let Some(blocker) = grid
                .iter()
                .filter(|(y, x, _)| (*y == pos.0) && (*x < pos.1))
                .last()
            {
                (blocker.1 + 1..=pos.1)
                    .map(|x| (pos.0, x))
                    .for_each(|coord| {
                        visited.insert(coord);
                    });
                *pos = (blocker.0, blocker.1 + 1);
                return false;
            }
            (0..=pos.1).map(|x| (pos.0, x)).for_each(|coord| {
                visited.insert(coord);
            });
            true
        }
        _ => panic!("Should not be a value over 3!"),
    }
}
