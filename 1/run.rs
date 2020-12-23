use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut numbers = HashSet::new();

    for a in contents.split("\n") {
        if a == "" {
            break
        }
        let as_int = a.parse::<i32>().unwrap();
        numbers.insert(as_int);
    }
    let target_num = 2020;
    for num in &numbers {
        for num2 in &numbers {
            let seeking = target_num - num - num2;
            if numbers.contains(&seeking) {
                    println!("{}: {} {} {}", num * num2 * seeking, num, num2, seeking);
            }
        }
    }
}
