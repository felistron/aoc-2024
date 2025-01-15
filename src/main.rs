mod day_1 {
    use std::str::FromStr;

    pub fn run(input: &str) {
        println!("======== Day 1 ========");

        let mut a: Vec<u32> = Vec::new();
        let mut b: Vec<u32> = Vec::new();

        let lines: Vec<&str> = input.split('\n').collect();

        for line in lines {
            let numbers: Vec<&str> = line.split_ascii_whitespace().collect();

            if numbers.len() == 0 {
                continue;
            }

            a.push(u32::from_str(numbers[0]).expect("Unreachable"));
            b.push(u32::from_str(numbers[1]).expect("Unreachable"));
        }

        assert_eq!(a.len(), b.len());

        a.sort();
        b.sort();

        let mut total_distance: u32 = 0;

        for (x, y) in a.iter().zip(b.clone()) {
            total_distance += x.abs_diff(y);
        }

        println!("Part 1: {total_distance}");

        let mut similarity_score: u32 = 0;

        for x in a.clone() {
            let mut count: u32 = 0;

            for y in b.clone() {
                if y == x {
                    count += 1;
                }
            }

            similarity_score += count * x;
        }

        println!("Part 2: {similarity_score}");
    }
}

mod day_2 {
    use std::str::FromStr;

    fn is_safe(report: &Vec<i32>) -> bool {
        let mut safe: bool = true;
        let mut inc: i32 = 0;

        for i in 1..report.len() {
            let dl: i32 = report[i] - report[i - 1];

            if dl.abs() < 1 || dl.abs() > 3 {
                safe = false;
            }

            if (inc * dl) < 0 {
                safe = false;
            }

            inc = dl;
        }

        safe
    }

    pub fn run(input: &str) {
        println!("======== Day 2 ========");

        let reports: Vec<&str> = input.split("\n").collect();

        let mut safe_reports: u32 = 0;
        let mut safe_dampener_reports: u32 = 0;

        for report in reports.iter() {
            if report.is_empty() {
                continue;
            }

            let levels: Vec<i32> = report
                .split(" ")
                .map(|r| i32::from_str(r)
                    .expect("Unreachable"))
                .collect();

            if is_safe(&levels) {
                safe_reports += 1;
                continue;
            }

            // part 2
            let mut safe: bool = false;

            for i in 0..levels.len() {
                let mut levels_copy = levels.clone();
                levels_copy.remove(i);

                if is_safe(&levels_copy) {
                    safe = true;
                    break;
                }
            }

            if safe {
                safe_dampener_reports += 1;
            }
        }

        println!("Part 1: {}", safe_reports);
        println!("Part 2: {}", safe_reports + safe_dampener_reports);
    }
}

fn print_aoc() {
    println!("~~~~~~~~~~~~~~~~~~~~~~~");
    println!("# Advent of code 2024 #");
    println!("~~~~~~~~~~~~~~~~~~~~~~~");

    // overcomplicated x-mas tree
    let size: usize = 5;
    for i in 0..size {
        print!("      ");
        for _ in 0..(size-i) {
            print!(" ");
        }
        for _ in 0..(2*i+1) {
            print!("*");
        }
        println!();
    }
    for _ in 0..(size/2) {
        print!("      ");
        for _ in 0..size {
            print!(" ");
        }
        println!("!")
    }

    println!("~~^^^^~~~~^^~~~~~~^^^~~");
    println!("^^~~~^^^~~~~^^^~~~~~^^~");

    println!();
}

fn main() {
    print_aoc();

    day_1::run(include_str!("../assets/input_d1.txt"));
    day_2::run(include_str!("../assets/input_d2.txt"));
}
