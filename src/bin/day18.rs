use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::{thread, time};

#[derive(PartialEq, Debug)]
enum OpResult<'a> {
    // The result of a line operation: nothing, a sound played, or a jump
    Void,
    Sound(isize),
    Recover(&'a str),
    Jump(isize),
}

fn execute1<'a>(line: &'a str, registers: &mut HashMap<&'a str, isize>) -> OpResult<'a> {
    let mut words = line.split_whitespace();
    let op = words.next().unwrap();
    let x_word = words.next().unwrap_or("a");
    let x_value = x_word.parse::<isize>().unwrap_or(
        *registers.get(x_word).unwrap_or(&0),
    );
    let y_word = words.next().unwrap_or("0");
    let y_value = y_word.parse::<isize>().unwrap_or(
        *registers.get(y_word).unwrap_or(&0),
    );
    let mut res = OpResult::Void;
    match op {
        "snd" => res = OpResult::Sound(x_value),
        "set" => {
            registers.insert(x_word, y_value);
        }
        "add" => {
            registers.insert(x_word, x_value + y_value);
        }
        "mul" => {
            registers.insert(x_word, x_value * y_value);
        }
        "mod" => {
            registers.insert(x_word, x_value % y_value);
        }
        "rcv" => {
            if x_value != 0 {
                res = OpResult::Recover(x_word);
            }
        }
        "jgz" => {
            if x_value > 0 {
                res = OpResult::Jump(y_value);
            }
        }
        _ => (),
    }
    res
}

fn solve1(input: &str) -> isize {
    let mut registers = HashMap::new();
    let mut last_sound = 0;
    let mut i = 0;
    let lines: Vec<&str> = input.lines().collect();
    loop {
        match execute1(lines[i as usize], &mut registers) {
            OpResult::Jump(x) => i += x - 1,
            OpResult::Sound(x) => last_sound = x,
            OpResult::Recover(_) => break,
            OpResult::Void => (),
        }
        i += 1;
    }
    last_sound
}

fn execute2<'a>(line: &'a str, registers: &mut HashMap<&'a str, isize>) -> OpResult<'a> {
    let mut words = line.split_whitespace();
    let op = words.next().unwrap();
    let x_word = words.next().unwrap_or("a");
    let x_value = x_word.parse::<isize>().unwrap_or(
        *registers.get(x_word).unwrap_or(&0),
    );
    let y_word = words.next().unwrap_or("0");
    let y_value = y_word.parse::<isize>().unwrap_or(
        *registers.get(y_word).unwrap_or(&0),
    );
    let mut res = OpResult::Void;
    match op {
        "snd" => res = OpResult::Sound(x_value),
        "set" => {
            registers.insert(x_word, y_value);
        }
        "add" => {
            registers.insert(x_word, x_value + y_value);
        }
        "mul" => {
            registers.insert(x_word, x_value * y_value);
        }
        "mod" => {
            registers.insert(x_word, x_value % y_value);
        }
        "rcv" => res = OpResult::Recover(x_word),
        "jgz" => {
            if x_value > 0 {
                res = OpResult::Jump(y_value);
            }
        }
        _ => (),
    }
    res
}

#[derive(PartialEq, Debug)]
enum State {
    Running(usize),
    Waiting,
}

fn run(
    input: Arc<str>,
    number: isize,
    tx: mpsc::Sender<isize>,
    rx: mpsc::Receiver<isize>,
    ty: mpsc::Sender<State>,
) {
    let mut registers = HashMap::new();
    registers.insert("p", number);
    let mut i = 0;
    let mut sent = 0;
    let lines: Vec<&str> = input.lines().collect();
    loop {
        match execute2(lines[i as usize], &mut registers) {
            OpResult::Jump(x) => i += x - 1,
            OpResult::Sound(x) => {
                tx.send(x).ok();
                sent += 1;
                ty.send(State::Running(sent)).ok();
            }
            OpResult::Recover(x) => {
                ty.send(State::Waiting).ok();
                registers.insert(x, rx.recv().unwrap());
                ty.send(State::Running(sent)).ok();
            }
            OpResult::Void => (),
        }
        i += 1;
    }
}

fn solve2(input: &str) -> usize {
    let (t0, r0) = mpsc::channel(); // channel 0 -> 1
    let (t1, r1) = mpsc::channel(); // channel 1 -> 0
    let (t2, r2) = mpsc::channel(); // Channel sending infos from runnner 0
    let (t3, r3) = mpsc::channel(); // Channel sending infos from runnner 1
    let input_ref0 = Arc::from(input);
    let input_ref1 = Arc::clone(&input_ref0);
    thread::spawn(|| run(input_ref0, 0, t0, r1, t2));
    thread::spawn(|| run(input_ref1, 1, t1, r0, t3));
    let mut state0 = State::Running(0);
    let mut state1 = State::Running(1);
    let mut sent_by_1 = 0;
    loop {
        let mut updated = false;
        for state in r2.try_iter() {
            state0 = state;
            updated = true;
        }
        for state in r3.try_iter() {
            state1 = state;
            updated = true;
            if let State::Running(n) = state1 {
                sent_by_1 = n
            }
        }
        if !updated && state0 == State::Waiting && state1 == State::Waiting {
            break;
        }
        thread::sleep(time::Duration::from_millis(1));
    }
    sent_by_1
}


fn main() {
    let mut f = File::open(Path::new("input/day18.txt")).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();
    let p1 = solve1(&input);
    let p2 = solve2(&input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
rcv a";

    #[test]
    fn execute_test() {
        let mut regs = HashMap::new();
        let mut lines = INPUT.lines();
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(regs.get("a"), Some(&1));
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(regs.get("a"), Some(&3));
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(regs.get("a"), Some(&9));
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(regs.get("a"), Some(&4));
        assert_eq!(
            execute1(lines.next().unwrap(), &mut regs),
            OpResult::Sound(4)
        );
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(execute1(lines.next().unwrap(), &mut regs), OpResult::Void);
        assert_eq!(
            execute1(lines.next().unwrap(), &mut regs),
            OpResult::Jump(-2)
        );
        assert_eq!(
            execute1(lines.next().unwrap(), &mut regs),
            OpResult::Recover("a")
        );
    }

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(INPUT), 4);
    }

    #[test]
    fn solve2_test() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(solve2(input), 3);
    }
}
