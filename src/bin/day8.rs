use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn execute(line: &str, registers: &mut HashMap<String, isize>, max: isize) -> isize {
    let words: Vec<&str> = line.split_whitespace().collect();
    // test condition
    let cond_reg_val = *registers.get(words[4]).unwrap_or(&0);
    let cond_val: isize = words[6].parse().unwrap();
    if match words[5] {
        "==" => cond_reg_val == cond_val,
        "!=" => cond_reg_val != cond_val,
        ">=" => cond_reg_val >= cond_val,
        "<=" => cond_reg_val <= cond_val,
        ">" => cond_reg_val > cond_val,
        "<" => cond_reg_val < cond_val,
        _ => false,
    }
    {
        let reg_value = *registers.get(words[0]).unwrap_or(&0);
        let mut inc: isize = words[2].parse().unwrap();
        if words[1] == "dec" {
            inc = -inc
        }
        let new = reg_value + inc;
        let new_max = new.max(max);
        registers.insert(words[0].into(), new);
        new_max
    } else {
        max
    }
}

fn solve(input: &str) -> (isize, isize) {
    let mut registers = HashMap::new();
    let mut max = 0;
    for line in input.lines() {
        max = execute(line, &mut registers, max);
    }
    (*registers.values().max().unwrap_or(&0), max)
}

fn main() {
    let mut f = File::open(Path::new("input/day8.txt")).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();
    let (p1, p2) = solve(&input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn execute_test() {
        let mut regs = HashMap::new();
        let mut lines = INPUT.lines();
        assert_eq!(execute(lines.next().unwrap(), &mut regs, 0), 0);
        assert_eq!(regs.get("b"), None);
        assert_eq!(execute(lines.next().unwrap(), &mut regs, 0), 1);
        assert_eq!(regs.get("a"), Some(&1));
        assert_eq!(execute(lines.next().unwrap(), &mut regs, 1), 10);
        assert_eq!(regs.get("c"), Some(&10));
        assert_eq!(execute(lines.next().unwrap(), &mut regs, 10), 10);
        assert_eq!(regs.get("c"), Some(&-10));
    }

    #[test]
    fn solve_test() {
        assert_eq!(solve(INPUT), (1, 10));
    }

}
