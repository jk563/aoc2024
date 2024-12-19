use std::fs::read_to_string;

pub struct Day02Solver {
    pub input_path: String,
}

impl Day02Solver {
    pub fn solve_part_one(&self) -> usize {
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

        reports.iter().filter(|report| report_is_good(&report)).count()
    }

    pub fn solve_part_two(&self) -> usize {
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

        // Cycle through each report with one item at a time removed, if any pass, the whole report
        // is good
        reports.iter()
                .filter(|report|
                    report.iter().enumerate().any(|(idx, _)| {
                        let mut sub_report = report.clone().to_owned();
                        sub_report.remove(idx);
                        report_is_good(&sub_report)
                    }))
                .count()
    }
}

fn report_is_good(report: &Vec<u64>) -> bool {
    // If two consecutive values are equal the report is deemed unsafe, check the first two as if
    // they are equal we can exit early, if they are not equal we can determine where they ascend 
    // or descend
    if report[0] == report[1] { return false };     

    if report[0] < report[1] { // Ascending integers
        report
            .windows(2) // Take each consecutive pair of digits
            .all( // Test all pairs meet the criteria
                |window|
                window[0] < window[1] // Successive numbers are ascending
                && window[1] - window[0] <= 3 // Difference of 3 or less
            ) // Report is good if all checks pass for all pairs
    } else { // Descending integers
        report
            .windows(2) // Take each consecutive pair of digits
            .all( // Test all pairs meet the criteria
                |window|
                window[0] > window[1] // Successive numbers are ascending
                && window[0] - window[1] <= 3 // Difference of 3 or less
            ) // Report is good if all checks pass for all pairs
    }
}
