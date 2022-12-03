use std::fs;

fn get_common_item(s: &str) -> u32 {
    let l = s.chars().count();
    let half = l/2;
    let s1 = &s[..half];
    let s2 = &s[s.len()/2..];

    for c in s1.chars() { 
        if s2.find(c) != None {
            return char_to_int(c);
        }
    }
    return 0;
}

fn char_to_int(c: char) -> u32 {
    if c as u32 > 90 { //lowercase
        return c as u32 - 'a' as u32 + 1;
    } else { //uppercase
        return c as u32 - 'A' as u32 + 27;
    }
}

fn main() {
    let contents = fs::read_to_string("sacks.txt")
        .expect("Should have been able to read the file");
    
    let priorities: _ = contents.lines().map(|x| get_common_item(x)).sum::<u32>();
    println!("{}", priorities)
}