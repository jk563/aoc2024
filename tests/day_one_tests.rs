use aoc2024::solvers::day01::Day01Solver;

#[test]
fn part_one_example() {
    let expected_solution = 11;

    let solver = Day01Solver {
        input_path: String::from("tests/day-one-example-input.txt")
    };
    let actual_solution = solver.solve_part_one();

    assert_eq!(actual_solution, expected_solution);
}
