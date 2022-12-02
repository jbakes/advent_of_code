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

    let mut elf_calories = Vec::<u32>::new();

    for elf in elves {
        let calories = elf.lines().map(|x| x.parse::<u32>().unwrap());
        elf_calories.push(calories.sum::<u32>());
    }

    // Use this for first star
    // let max_value = elf_calories.iter().max();
    // match max_value {
    //     Some(max) => println!( "Max value: {}", max),
    //     None      => println!( "Vector is empty"),
    // }

    // Use this for the second star
    elf_calories.sort();
    // let l = elf_calories.len();
    // let top_three = elf_calories[l-1] + elf_calories[l-2] + elf_calories[l-3];
    // println!("{}", top_three);

    println!("{}", elf_calories.into_iter().rev().take(3).sum::<u32>());
    
}