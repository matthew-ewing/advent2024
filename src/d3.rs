use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;


pub fn d3() {
    let file_path = "/Users/matthewe/GitHub/advent2024/input/d3.txt";

    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));
    let mut lines: String = String::new();
    for line in reader.lines() {
        lines += &line.unwrap();
    }

    // Regex solution
    let re_pattern = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let matches = re_pattern.find_iter(&lines);
    let mut mult = true;
    let mut part_1_result = 0;
    let mut part_2_result = 0;
    for m in matches {
        let m_str = m.as_str();
        if m_str == "do()" {
            mult = true;
        } else if m_str == "don't()" {
            mult = false;
        } else {
            let mult_res = process_ins((m_str).to_string());
            if mult {
                part_2_result += mult_res;
            } else {
                part_1_result += mult_res;
            }
        }
    }

    // Custom parser, because why not....
    let mut commands: Vec<String> = Vec::new();
    
    let mut split_dont: Vec<String> = lines.split("don't()").map(str::to_string).collect();
    // special case.. handle first section up to initial don't
    commands.push(split_dont[0].clone());
    split_dont.remove(0);

    for item in &split_dont {
        let mut pass2: Vec<String> = item.split("do()").map(str::to_string).collect();
        // Remove first element (this is section from the don't to the first do)
        pass2.remove(0);
        // Add all remaining elements to the commands vector
        commands.extend(pass2);
    }

    let mut sum = 0;
    for command in commands {
        let ins_vec: Vec<String> = command.split("m").map(str::to_string).collect();
        for item in ins_vec{
            let result = process_ins(item);
            sum += result
        }
        
    }
    println!("Regex Part 1: {}, Regex Part 2: {}; Custom Parser Part 2: {}", part_1_result, part_2_result, sum);
}

fn process_ins(s: String) -> i32 {
    let start: Vec<String> = s.split("(").map(str::to_string).collect();
    if start[0] == "ul" || start[0] == "mul"{
        let nums: Vec<String> = start[1].split(",").map(str::to_string).collect();
        if nums.len() >= 2 {
            let num1 = nums[0].clone().parse::<i32>();
            let num2_vec: Vec<String> = nums[1].split(")").map(str::to_string).collect();
            let num2 = num2_vec[0].clone().parse::<i32>();
            if num1.is_ok() && num2.is_ok() {
                return num1.unwrap() * num2.unwrap();
            }
        }
    }
    return 0;
}
