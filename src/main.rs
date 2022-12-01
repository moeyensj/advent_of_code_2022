use std::fs;

fn main() {
    day_01();
}

fn day_01() {
    // I should learn how to use ndarray...
    let contents = fs::read_to_string("day01_input.txt")
        .expect("file");

    let lines = contents.split("\n");
    let mut _current_elf = 1;
    let mut calories = 0;
    let mut carried_calories = Vec::<i32>::new();
    for line in lines {
        if line == "" {
            carried_calories.push(calories);
            _current_elf += 1;
            calories = 0;
        } else {
            calories += line.parse::<i32>().unwrap();
        }
    }

    carried_calories.sort();
    let n: usize = carried_calories.len().try_into().unwrap();
    let sum1: i32 = carried_calories[n - 1];
    let sum3: i32 = carried_calories[n - 1] + carried_calories[n - 2] + carried_calories[n - 3];
    println!("{:?}", sum1);
    println!("{:?}", sum3);
}