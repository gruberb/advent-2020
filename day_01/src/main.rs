use std::fs;

fn main() {
    let contents = fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong reading the file");

    let mut contents: Vec<usize> = contents.lines().filter_map(|s| Some(s.parse::<usize>().expect("not a number"))).collect();
    contents.sort();

    let (first, second) = find_two(&contents, 2020);

    println!("---------FIND TWO-----------");
    println!("First: {}, Second: {}, Sum: {}", first, second, first * second);
    println!("----------------------------");

    let (first, second, third) = find_three(&contents, 2020); 
    
    println!("---------FIND THREE-----------");
    println!("First: {}, Second: {}, Third: {}, Sum: {}", first, second, third, first * second * third);
    println!("----------------------------");
}

fn find_two(list: &Vec<usize>, sum: usize) -> (usize, usize) {
    let mut l = 0;
    let mut r = list.len() -1;
    while l < r {
        if list[l] + list[r] == sum {
            return (list[l], list[r]);      
        } else if list[l] + list[r] < sum {
            l += 1;
        } else {
            r -= 1;
        }
    }

    return (0, 0);
}

fn find_three(list: &Vec<usize>, sum: usize) -> (usize, usize, usize) {
    for (i, _) in list.iter().enumerate() {
        let mut l = i + 1;
        let mut r = list.len() - 1;

        while l < r {
            if list[i] + list[l] + list[r] == sum {
                return (list[i], list[l], list[r]);
            } else if list[i] + list[l] + list[r] < sum {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    (0,0,0)
}
