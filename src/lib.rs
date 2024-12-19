pub mod solvers;

use crate::solvers::*;

pub fn run() {
    let day_01_solver = Day01Solver{
        input_path: String::from("puzzle-inputs/day01.txt")
    };
    println!("Day 01 Part One: {}", day_01_solver.solve_part_one());
    println!("Day 01 Part Two: {}", day_01_solver.solve_part_two());

    println!();

    let day_02_solver = Day02Solver{
        input_path: String::from("puzzle-inputs/day02.txt")
    };
    println!("Day 02 Part One: {}", day_02_solver.solve_part_one());
    println!("Day 02 Part Tne: {}", day_02_solver.solve_part_two());
}

