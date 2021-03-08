use std::fs;

fn main() {
    let contents = fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong reading the file");

    let contents: Vec<&str> = contents.lines().collect();

    let old = old_policy(&contents);
    let new = new_policy(&contents);

    println!("{}, {}", old, new);

}

fn old_policy(policy: &Vec<&str>) -> i64 {
    let mut valid = 0;

    for p in policy {
        let f = p.find("-").expect("");
        let min = p[0..f].parse::<usize>().expect("");
        
        let s = f+1;
        let max = p[s..p.find(" ").expect("")].parse::<usize>().expect("");
        
        let start = p.find(":").expect("") + 2;
        let password = &p[start..p.len()];
        
        let rule = &p[p.find(" ").expect("") + 1..p.find(":").expect("")];
        
        let count = password.matches(rule).count();

        if count <= max && count >= min {
            valid += 1;
        }
    }

    valid
}

fn new_policy(policy: &Vec<&str>) -> i64 {
    let mut valid = 0;

    for p in policy {
        let f = p.find("-").expect("");
        let min = p[0..f].parse::<usize>().expect("");
        
        let s = f+1;
        let max = p[s..p.find(" ").expect("")].parse::<usize>().expect("");
        
        let start = p.find(":").expect("") + 2;
        let password = &p[start..p.len()];

        let rule = &p.chars().nth(p.find(":").expect("")-1).expect("");

        if &password.chars().nth(min-1).unwrap() == rule && &password.chars().nth(max-1).unwrap() != rule {
            valid += 1;
        } else if &password.chars().nth(min-1).unwrap() != rule && &password.chars().nth(max-1).unwrap() == rule {
                valid += 1;
        }
    }

    valid
}