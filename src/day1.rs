use crate::globalfuncs;

pub fn part1() {
    let numbers = globalfuncs::read_data("data/day1_input", "\n");
    println!("{}", numbers.iter().fold(0, |acc, fuel| acc + (fuel/3) -2 ))
}

pub fn part2() {
    let numbers = globalfuncs::read_data("data/day1_input","\n");
    println!("{}", numbers.iter().fold(0, |acc, fuel| acc + mass_to_zero(*fuel)))
}

fn fuel_required(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn mass_to_zero( mass: i32) -> i32 {
    let mut sum = 0;
    let mass = fuel_required(mass);
    sum += mass;
    if mass >= 9 {
        sum += mass_to_zero(mass)
    }
    sum
}