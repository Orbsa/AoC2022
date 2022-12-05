use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
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
    print!("{} elves. Max Calories: ",elfs.len());
    let maxc = elfs.iter().max();
    match maxc {
        Some(maxc) => println!("{}",maxc),
        None => println!("")
    }
}
