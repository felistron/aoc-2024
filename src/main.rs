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

mod day_3 {
    use std::str::FromStr;

    struct Input {
        string: Vec<char>,
        current_index: usize,
    }

    impl Input {
        fn new(input: &str) -> Input {
            Input {
                string: input.chars().collect(),
                current_index: 0
            }
        }

        fn next(&mut self) -> Option<char> {
            if self.has_next() {
                let c = self.string[self.current_index];
                self.current_index += 1;
                Some(c)
            } else {
                None
            }
        }

        fn next_number(&mut self) -> Option<i32> {
            let mut buffer = String::new();

            while self.string[self.current_index].is_digit(10) {
                buffer.push(self.string[self.current_index]);
                self.current_index += 1;
            }

            match i32::from_str(&buffer) {
                Ok(number) => Some(number),
                Err(_) => None
            }
        }

        fn has_next(&self) -> bool {
            self.current_index + 1 < self.string.len()
        }

        fn expect(&mut self, c: char) -> bool {
            if self.string[self.current_index] == c {
                self.current_index += 1;
                true
            } else {
                false
            }
        }
    }

    pub fn run(input: &str) {
        println!("======== Day 3 ========");

        let mut data = Input::new(input);
        let mut result: i32 = 0;
        let mut enabled_result: i32 = 0;
        let mut instructions_enabled: bool = true;

        while data.has_next() {
            (|| {
                if !data.expect('d') { return; }
                if !data.expect('o') { return; }

                if data.expect('(') {
                    if !data.expect(')') { return; }
                    instructions_enabled = true;
                } else {
                    if !data.expect('n') { return; }
                    if !data.expect('\'') { return; }
                    if !data.expect('t') { return; }
                    if !data.expect('(') { return; }
                    if !data.expect(')') { return; }

                    instructions_enabled = false;
                }
            })();

            (|| {
                if !data.expect('m') { return; }
                if !data.expect('u') { return; }
                if !data.expect('l') { return; }
                if !data.expect('(') { return; }

                let a = match data.next_number() {
                    Some(number) => number,
                    None => return
                };

                if !data.expect(',') { return; }

                let b = match data.next_number() {
                    Some(number) => number,
                    None => return
                };

                if !data.expect(')') { return; }

                if instructions_enabled {
                    enabled_result += a * b;
                }

                result += a * b;
            })();

            data.next();
        }

        println!("Part 1: {}", result);
        // TODO: Part 2 works just fine with the example but it doesn't with the input.
        // I'll come back to fix it some day :)
        // println!("Part 2: {}", enabled_result);
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
    day_3::run(include_str!("../assets/input_d3.txt"));
}
