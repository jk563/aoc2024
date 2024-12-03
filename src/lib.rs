pub mod solvers;

use crate::solvers::*;

pub fn run() {
    println!("Day 01 Part One: {}", Day01Solver{
        input_path: String::from("puzzle-inputs/day01.txt")
    }.solve_part_one());
    println!("Day 01 Part Two: {}", Day01Solver{
        input_path: String::from("puzzle-inputs/day01.txt")
    }.solve_part_two());
}

