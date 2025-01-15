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
    let input = include_str!("../assets/input.txt");

    print_aoc();
    day_1::run(input);
}
