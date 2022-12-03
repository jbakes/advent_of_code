use std::fs;

fn rps(line: &str) -> u32{
    if line == "A X" {
        return 1 + 3;
    } else if line == "B X" {
        return 1 + 0;
    } else if line == "C X" {
        return 1 + 6;
    } else if line == "A Y" {
        return 2 + 6;
    } else if line == "B Y" {
        return 2 + 3;
    } else if line == "C Y" {
        return 2 + 0;
    } else if line == "A Z" {
        return 3 + 0;
    } else if line == "B Z" {
        return 3 + 6;
    } else if line == "C Z" {
        return 3 + 3;
    } else {
        return 0
    }
}

fn main() {
    let contents = fs::read_to_string("data/rps.txt")
        .expect("Should have been able to read the file");

    let points: _ = contents.lines().map(|x| rps(x)).sum::<u32>();
    println!("{}", points)
}
