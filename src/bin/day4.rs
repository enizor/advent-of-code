use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::path::Path;


fn main() {
    match solve() {
        Ok((sum1, sum2)) => println!("Part 1: {} | Part 2: {}", sum1, sum2),
        Err(err) => println!("/!\\ Error! {}", err.to_string()),
    }
}

fn solve() -> Result<(u32, u32), Error> {
    let f = File::open(Path::new("input/day4.txt"))?;
    let file = BufReader::new(&f);
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in file.lines() {
        let x = line.unwrap();
        if valid_passphrase(&x) { sum1 += 1 }
    }
    Ok((sum1, sum2))
}

fn valid_passphrase(s: &str) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    (&words).iter().filter(|w| contains(*w, &words)).next().is_none()
}

fn contains(word: &str, collection: &Vec<&str>) -> bool {
    !collection.iter().filter( |w| *word == ***w ).nth(1).is_none()
}
