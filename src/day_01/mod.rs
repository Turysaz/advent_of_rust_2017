use utils::char_to_int;

fn make_circular(input : &str) -> String {
    let bytes = input.as_bytes();
    let first = bytes[0] as char;
    let mut circ = String::from(input);
    circ.push(first);
    circ
}

pub fn solve_part_one(input : String) -> u32{

    let input = &make_circular(&(input.trim()));
    let input = input.as_bytes();

    let mut index = 0;
    let mut checksum = 0;

    while index < input.len() - 1{
        let left_n = char_to_int(input[index] as char);
        let right_n = char_to_int(input[index + 1] as char);
        if left_n == right_n{
            checksum += left_n;
        }
        index += 1;
    }

    checksum
}

pub fn solve_part_two(input : String) -> u32{
    let input = input.as_bytes();
    let mut checksum = 0;
    let mut index_left = 0;
    let mut index_right = input.len() / 2;
    while index_left < input.len() / 2 {
        let left = char_to_int(input[index_left] as char);
        let right = char_to_int(input[index_right] as char);
        if left == right{
            checksum += left*2;
        }
        index_left += 1;
        index_right += 1;
    }

    checksum
}
