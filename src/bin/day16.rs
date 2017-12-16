use std::fs::File;
use std::io::Read;
use std::path::Path;

fn dance(input: &str, dancers: &mut Vec<usize>) {
    for mut step in input.split(",").map(|s| s.chars()) {
        match step.next().unwrap() {
            's' => {
                spin(step.collect::<String>().parse().unwrap(), dancers);
            }
            'x' => {
                let nums = step.collect::<String>()
                    .split('/')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                exchange(nums[0], nums[1], dancers)
            }
            'p' => partner(step.next().unwrap(), step.nth(1).unwrap(), dancers),
            _ => (),
        }
    }
}

fn spin(n: usize, dancers: &mut Vec<usize>) {
    let l = dancers.len();
    let save = dancers[l - n..].to_owned();
    for i in 1..l - n + 1 {
        dancers[l - i] = dancers[(l - i - n) % l]
    }
    for i in 0..n {
        dancers[i] = save[i]
    }
}

fn exchange(i: usize, j: usize, dancers: &mut Vec<usize>) {
    dancers.swap(i, j);
}

fn partner(x: char, y: char, dancers: &mut Vec<usize>) {
    let x1 = ((x as u8) - 97) as usize;
    let y1 = ((y as u8) - 97) as usize;
    let i = dancers.iter().position(|&z| z == x1).unwrap();
    let j = dancers.iter().position(|&z| z == y1).unwrap();
    dancers.swap(i, j);
}

fn display(dancers: &[usize]) -> String {
    dancers.iter().map(|&x| char::from(x as u8 + 97)).collect()
}

fn solve(input: &str, repetitions: usize) -> String {
    let mut dancers = (0..16).collect::<Vec<usize>>();
    let mut seen = vec![dancers.clone()];
    let mut i = 0;
    let mut remaining = 0;
    while i < repetitions {
        dance(input, &mut dancers);
        i += 1;
        if seen.iter().any(|x| *x == dancers) {
            println!("Found a repetition! n°{}", i);
            remaining = repetitions % i;
            break;
        } else {
            seen.push(dancers.clone())
        }
    }
    for _ in 0..remaining {
        dance(input, &mut dancers);
    }
    display(&dancers)
}

fn main() {
    let mut f = File::open(Path::new("input/day16.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim();
    let mut dancers = (0..16).collect::<Vec<usize>>();
    dance(input, &mut dancers);
    let p1 = display(&dancers);
    let p2 = solve(input, 1_000_000_000);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dance_test() {
        let mut dancers = (0..5).collect::<Vec<usize>>();
        dance("s1,x3/4,pe/b", &mut dancers);
        assert_eq!(display(&dancers), "baedc".to_owned())
    }

    #[test]
    fn solve_test() {
        let mut dancers = (0..16).collect::<Vec<usize>>();
        dance("s1,x13/4,pe/b,s1,x13/4,pe/b", &mut dancers);
        assert_eq!(display(&dancers), solve("s1,x13/4,pe/b", 2))
    }
}
