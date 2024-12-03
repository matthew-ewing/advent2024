use std::io::{BufRead, BufReader};
use std::fs::File;
use itertools::Itertools;
use std::ops::Not;

pub fn d2() {
    let file_path = "/Users/matthewe/GitHub/advent2024/input/d2.txt";

    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));
    
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let nums: Vec<String> = line.unwrap().split_whitespace().map(str::to_string).collect();
        let mut report: Vec<i32> = Vec::new();
        for num in nums {
            report.push(num.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    let mut not_safe = 0;
    for r in reports.clone() {
        let result = process_report(r.clone());
        if result.0.not() {            
            // Part 1
            // not_safe += 1;

            // Part 2
            // Try again removing first bad element
            let mut try_1 = r.clone();
            try_1.remove(result.1);
            let try_1_res = process_report(try_1);
            if try_1_res.0.not() {
                // Try again removing second bad element
                let mut try_2 = r.clone();
                try_2.remove(result.1 + 1);
                let try_2_res = process_report(try_2);

                if try_2_res.0.not() {
                    not_safe += 1;
                }
            }
        }
    }
    println!("Safe: {}", reports.len() - not_safe)

}

fn process_report(r: Vec<i32>) -> (bool, usize) {
    let mut inc : bool = false;
    for (idx, (&elem, &next)) in r.iter().tuple_windows().enumerate() {
        // record if report should be increasing or decreasing
        if idx == 0 {
            inc = if elem > next {false} else {true};
        }
        // verify report
        if verify(elem, next, inc).not() {
            return (false, idx)
        }
    }
    return (true, usize::MAX);
}

fn verify(x: i32, y: i32, inc: bool) -> bool {
    let diff = (x-y).abs();
    if (diff > 3 || diff < 1) || (inc && x > y) || (inc.not() && x < y) {
        return false;
    }
    return true;
}