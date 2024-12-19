use aoc2024::solvers::day02::Day02Solver;

#[test]
fn part_one_example() {
    let expected_solution = 2;
    
    let solver = Day02Solver {
        input_path: String::from("tests/day-two-example-input.txt")
        };
    let actual_solution = solver.solve_part_one();

    assert_eq!(actual_solution, expected_solution);
}

#[test]
fn part_two_example() {
    let expected_solution = 4;
    
    let solver = Day02Solver {
        input_path: String::from("tests/day-two-example-input.txt")
        };
    let actual_solution = solver.solve_part_two();

    assert_eq!(actual_solution, expected_solution);
}

