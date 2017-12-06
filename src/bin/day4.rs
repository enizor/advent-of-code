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
        if valid_passphrase(&x) {
            sum1 += 1
        }
        if valid_passphrase2(&x) {
            sum2 += 1
        }
    }
    Ok((sum1, sum2))
}

fn valid_passphrase(s: &str) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    (&words)
        .iter()
        .filter(|w| contains(*w, &words))
        .next()
        .is_none()
}

fn contains(word: &str, collection: &Vec<&str>) -> bool {
    !collection.iter().filter(|w| *word == ***w).nth(1).is_none()
}

fn valid_passphrase2(s: &str) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    (&words)
        .iter()
        .filter(|w| contains_anagram(*w, &words))
        .next()
        .is_none()
}

fn contains_anagram(word: &str, collection: &Vec<&str>) -> bool {
    !collection
        .iter()
        .filter(|w| (is_anagram(word, w.to_string())))
        .nth(1)
        .is_none()
}

fn is_anagram(u: &str, mut v: String) -> bool {
    let mut res = true;
    for c in u.chars() {
        if let Some(n) = v.find(c) {
            v.remove(n);
        } else {
            res = false
        }
    }
    &v == "" && res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_test() {
        let inputs = [
            ("aa", vec!["aa", "bb", "cc", "dd", "ee"]),
            ("aa", vec!["aa", "aa", "aa", "aa"]),
            ("aa", vec!["aa"]),
            ("aa", vec!["bb", "aa"]),
            ("aa", vec!["aa", "bb", "aa"]),
        ];
        let solutions = [false, true, false, false, true];
        for (&(word, ref phrase), solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(contains(word, &phrase), *solution)
        }
    }

    #[test]
    fn anagram_test() {
        let inputs = [
            ("a", "a"),
            ("ab", "ba"),
            ("ab", "abc"),
            ("abc", "ba"),
            ("revolutionfrancaise", "unvetocorselafinira"),
            ("tommarvoloriddle", "iamlordvoldemort"),
        ];
        let solutions = [true, true, false, false, true, true];
        for (&(word, phrase), solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(is_anagram(word, phrase.to_string()), *solution)
        }
    }

    #[test]
    fn valid1_test() {
        let inputs = ["aa bb cc dd ee", "aa bb cc dd aa", "aa bb cc dd aaa"];
        let solutions = [true, false, true];
        for (phrase, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(valid_passphrase(phrase), *solution)
        }
    }

    #[test]
    fn valid2_test() {
        let inputs = [
            "abcde fghij",
            "abcde xyz ecdab",
            "a ab abc abd abf abj",
            "iiii oiii ooii oooi oooo",
            "oiii ioii iioi iiio",
        ];
        let solutions = [true, false, true, true, false];
        for (phrase, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(valid_passphrase2(phrase), *solution)
        }
    }
}
