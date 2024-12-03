use std::fs::read_to_string;
use std::iter::zip;

pub struct Day01Solver {
    pub input_path: String,
}

impl Day01Solver {
    pub fn solve_part_one(&self) -> u64 {
        // Initialise lists to populate with location ISs
        let mut left_list: Vec<u64> = Vec::new();
        let mut right_list: Vec<u64> = Vec::new();

        // Populate location ID lists from input
        for line in read_to_string(&self.input_path).unwrap().lines() {
            let mut line_location_ids = line.split_whitespace();
            left_list.push(line_location_ids.next().unwrap().parse().unwrap());
            right_list.push(line_location_ids.next().unwrap().parse().unwrap());
        }

        // Sort the lists to compare IDs in order
        left_list.sort();
        right_list.sort();

        // Calcuate absolute difference between each pairing, keeping track of cumalative total
        let iterator = zip(left_list, right_list);
        let mut difference = 0;
        for pair in iterator {
            difference += pair.0.abs_diff(pair.1);
        }

        difference
    }

    pub fn solve_part_two(&self) -> u64 {
        // Initialise lists to populate with location ISs
        let mut left_list: Vec<u64> = Vec::new();
        let mut right_list: Vec<u64> = Vec::new();

        // Populate location ID lists from input
        for line in read_to_string(&self.input_path).unwrap().lines() {
            let mut line_location_ids = line.split_whitespace();
            left_list.push(line_location_ids.next().unwrap().parse().unwrap());
            right_list.push(line_location_ids.next().unwrap().parse().unwrap());
        }

        // Sort the lists to compare IDs in order
        left_list.sort();
        right_list.sort();

        // Calculate similarity score
        let mut similarity_score = 0;
            for left_location_id in left_list.iter() {
                similarity_score += *left_location_id * right_list.iter()
                          .filter(|&right_location_id|
                                  *right_location_id == *left_location_id)
                          .count() as u64;
            }
            similarity_score
    }
}
