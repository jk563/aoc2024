use std::fs::read_to_string;

pub struct Day02Solver {
    pub input_path: String,
}

impl Day02Solver {
    pub fn solve_part_one(&self) -> u64 {
        let reports: Vec<Vec<u64>> = read_to_string(&self.input_path)
            .unwrap() // Panic on file read errors
            .lines() // Iterate over each line of input
            .map( // For each line in the file
                |line| line.split_whitespace() // Split the line on whitespace
                    .map( // For each item on each line
                        |s| s.parse() // Parse as an integer
                                .unwrap() // Panic on parsing errors
                    )
                    .collect() // Create a vector of integers for the line
            )
            .collect(); // Create a vector containing each line's vector of integers

        let mut safe_count = 0;

        for report in reports  {
            if report[0] == report[1] { continue }; // If two consecutive values are equal the
                                                    // report is deemed unsafe
            
            if report[0] < report[1] { // Ascending integers
                safe_count += if report
                                    .windows(2) // Take each consecutive pair of digits
                                    .all( // Test all pairs meet the criteria
                                        |window| 
                                        window[0] < window[1] // Successive numbers are ascending
                                        && window[1] - window[0] <= 3 // Difference of 3 or less
                                    ) { 1 } else {0}; // Increment the safe count only if the tests
                                                      // all pass for all pairs
            }
            
            if report[0] > report[1] { // Descending integers
                safe_count += if report
                                    .windows(2) // Take each consecutive pair of digits
                                    .all( // Test all pairs meet the criteria
                                        |window| 
                                        window[0] > window[1] // Successive numbers are descending
                                        && window[0] - window[1] <= 3 // Difference of 3 or less
                                    ) { 1 } else {0}; // Increment the safe count only if the tests
                                                      // all pass for all pairs
            }
        }
        safe_count
    }
}
