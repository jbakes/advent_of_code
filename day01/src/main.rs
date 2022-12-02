use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split = contents.split("\n\n");
    
    let elves: Vec<&str> = split.collect();

    let mut elf_calories = Vec::<i64>::new();

    for elf in elves {
        // let calories: Vec<i64> = elf.lines().map(|x| x.parse::<i64>().unwrap()).collect();
        let calories = elf.lines().map(|x| x.parse::<i64>().unwrap());
        elf_calories.push(calories.sum::<i64>());
    }

    let max_value = elf_calories.iter().max();
    match max_value {
        Some(max) => println!( "Max value: {}", max),
        None      => println!( "Vector is empty"),
    }

}