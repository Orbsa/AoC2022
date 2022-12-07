use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}


fn day1_1() {
    let filename = "src/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut elfs:Vec<i32> = Vec::new();

    let mut cur_elf: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if String::is_empty(&line) {
            elfs.push(cur_elf);
            cur_elf = 0;
            continue;
        }
        let calories: i32 = line.parse().unwrap();
        cur_elf += calories;
    }
    println!("{} elves.",elfs.len());
    elfs.sort();
    elfs.reverse();
    println!("Max Calories: {}",elfs[0]);
    let p2: i32 = elfs.iter().take(3).sum();
    println!("Sum Calories of top 3 elves: {}",p2);
}


fn day2_1() {
    let input= read_input("src/input2");
    let scores: [i8; 3] = [3,0,6];
    let mut points: i32 = 0;
    for line in input.lines() {
        let ascii = line.as_bytes();
        let opp:i8 = (ascii[0]  - b'A'+1) as i8;
        let me:i8 = (ascii[2]  - b'X'+1) as i8;
        let i = (3 - me + opp) % 3;
        let score = scores[i as usize];
        let rdpoints = me as i32 + score as i32;
        points += rdpoints;
    }
    println!("Total Points: {}",points);
}

fn day2_2() {
    let input= read_input("src/input2");
    let mut points: i32 = 0;
    for line in input.lines() {
        let ascii = line.as_bytes();
        let opp:i8 = (ascii[0]  - b'A') as i8;
        let res:i8 = (ascii[2]  - b'X') as i8;
        let me = (3 + opp - 1 + res ) % 3 + 1;
        let rdpoints = (res * 3) as i32 + me as i32;
        points += rdpoints;
        //println!("{}||O:{} R:{} M:{} P:{}",line,opp,res,me,rdpoints);
    }
    println!("Total Points: {}",points);

}

fn main() {
    day2_2();
}