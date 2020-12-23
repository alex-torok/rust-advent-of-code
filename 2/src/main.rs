use std::fs;
use regex::Regex;

struct Password {
    min: u8,
    max: u8,
    c: char,
    password: String
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([^\n]+)").unwrap();

    let mut passwords = Vec::new();

    for cap in re.captures_iter(&contents) {
        let min = cap[1].parse::<u8>().unwrap();
        let max = cap[2].parse::<u8>().unwrap();
        let c = cap[3].chars().nth(0).unwrap();
        let password = cap[4].to_string();
        passwords.push(Password{
            min, max, c, password
        })
    }
    let mut correct = 0;
    for p in &passwords {
        let count = p.password.matches(p.c).count();
        if count >= p.min.into() && count <= p.max.into() {
            correct+=1;
        }
    }
    println!("pt1: {}", correct);

    let mut correct2 = 0;
    for p in passwords {
        println!("{}", p.password);
        let first = p.password.chars().nth(( p.min-1 ).into());
        let second = p.password.chars().nth(( p.max-1 ).into());

        let first_matches = first.is_some() && first.unwrap() == p.c;
        let second_matches = second.is_some() && second.unwrap() == p.c;

        if (first_matches || second_matches) && !(first_matches && second_matches){
            correct2 += 1
        }
    }
    println!("pt2: {}", correct2)
}
