use std::fs;

fn main() {
    let contents = fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong reading the file");

    let contents: Vec<u32> = contents.lines().filter_map(|s| Some(s.parse::<u32>().expect("not a number"))).collect();

    for (i, number) in contents.iter().enumerate() {
       let mut sum = 0;



    }
}
