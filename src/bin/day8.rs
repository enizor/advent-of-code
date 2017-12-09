use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn execute(line: &str, registers: &mut HashMap<String, isize>) {
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
        registers.insert(words[0].into(), reg_value + inc);
    }
}

fn solve1(input: &str) -> isize {
    let mut registers = HashMap::new();
    for line in input.lines() {
        execute(line, &mut registers)
    }
    *registers.values().max().unwrap_or(&0)
}

fn main() {
    let mut f = File::open(Path::new("input/day8.txt")).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();
    println!("Part 1: {}, Part 2: {}", solve1(&input), 0)
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
        execute(lines.next().unwrap(), &mut regs);
        assert_eq!(regs.get("b"), None);
        execute(lines.next().unwrap(), &mut regs);
        assert_eq!(regs.get("a"), Some(&1));
        execute(lines.next().unwrap(), &mut regs);
        assert_eq!(regs.get("c"), Some(&10));
        execute(lines.next().unwrap(), &mut regs);
        assert_eq!(regs.get("c"), Some(&-10));
    }

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(INPUT), 1);
    }

}
