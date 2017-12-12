use std::fs::File;
use std::io::Read;

pub fn read_input(path : &str) -> String{
    let mut file = File::open(path).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file");
    content
}

pub fn char_to_int(c : char) -> u32{
    let mut string = String::new();
    string.push(c);
    string.parse().expect("Cannot parse: Not a number!")
}

pub fn split_lines(input : &str) -> Vec<&str>{
    let mut lines : Vec<&str> = Vec::new();
    let bytes = input.as_bytes();
    let mut begin = 0;
    for (i, &b) in bytes.iter().enumerate(){
        if b == b'\n' {
            lines.push(&input[begin..i]);
            begin = i+1;
        }
    }
    lines
}

pub fn string_to_numbers(s : &str) -> Vec<u32>{
    let mut numbers_s : Vec<&str> = Vec::new();
    let mut numbers_n : Vec<u32> = Vec::new();

    let bytes = s.as_bytes();
    let mut begin = 0;

    for (i, &b) in bytes.iter().enumerate(){
        if b == b' ' || b == b'\t'{
            numbers_s.push(&s[begin..i]);
            begin += 1;
        }
    }

    for &n in numbers_s.iter(){
        numbers_n.push(n.parse().expect("Failed to parse number!"));
    }

    numbers_n
}
