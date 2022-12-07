use std::fs;

// fn get_common_item(s: &str) -> u32 {
//     let l = s.chars().count();
//     let half = l/2;
//     let s1 = &s[..half];
//     let s2 = &s[s.len()/2..];

//     for c in s1.chars() { 
//         if s2.find(c) != None {
//             return char_to_int(c);
//         }
//     }
//     return 0;
// }

fn get_common(s1: &str, s2: &str, s3: &str) -> u32 {
    for c in s1.chars() {
        if s2.find(c) != None {
            if s3.find(c) != None {
                return char_to_int(c);
            }
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
    
    let sack_lines = contents.lines();

    let mut i = 0;
    let mut s1 = "";
    let mut s2 = "";
    let mut priorities = 0;

    for l in sack_lines {
        if i % 3 == 0 {
            s1 = l;
        }
        if i % 3 == 1 {
            s2 = l;
        }        
        if i % 3 == 2 {
            let s3 = l;
            priorities += get_common(s1, s2, s3);
        }
        i += 1;
    }

    println!("{}", priorities)
}