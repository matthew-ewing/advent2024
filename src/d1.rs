use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn d1() {
    let file_path = "/Users/matthewe/GitHub/advent2024/input/d1.txt";

    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let nums: Vec<String> = line.unwrap().split_whitespace().map(str::to_string).collect();
        left.push(nums[0].parse::<i32>().unwrap());
        right.push(nums[1].parse::<i32>().unwrap());
    }
    
    left.sort();
    right.sort();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for (a, b) in left.into_iter().zip(right.clone()) {
        // Part 1 Solution
        let val1 = (a-b).abs();
        sum1 += val1;

        // Part 2 Solution
        let val2 = right.iter().filter(|&n| *n == a).count() as i32 * a;
        sum2 += val2
    }
    
    println!("{0}, {1}", sum1, sum2);
}
