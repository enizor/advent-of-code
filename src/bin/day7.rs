use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Tower {
    name: String,
    weight: usize,
    childs: Vec<Box<Tower>>,
}

impl Tower {
    fn insert(&mut self, t: Tower) {
        self.childs.push(Box::new(t));
    }
}

impl PartialEq for Tower {
    fn eq(&self, other: &Tower) -> bool {
        self.name == other.name
    }
}

// The second returned value is 'does the tower have children?'
fn new_tower(s: &str) -> (Tower, bool) {
    // remove junk
    let mut words = s.split_whitespace();
    let name = words.next().unwrap().into();
    let weight = words
        .next()
        .unwrap()
        .trim_matches(|p| p == '(' || p == ')')
        .parse::<usize>()
        .unwrap();
    (
        Tower {
            name: name,
            weight: weight,
            childs: vec![],
        },
        words.next().is_none(),
    )
}

fn find<T: PartialEq + std::fmt::Debug>(v: &[T], x: T) -> Option<usize> {
    let mut res = None;
    let mut i = 0;
    while i < v.len() && res.is_none() {
        if v[i] == x {
            res = Some(i)
        }
        i += 1;
    }
    res
}

fn find_children(s: &str, towers: &Vec<Tower>) -> Option<Vec<usize>> {
    let mut res = vec![];
    let words: Vec<&str> = s.split_whitespace().collect();
    for w in &words[3..] {
        res.push(find(
            towers,
            Tower {
                name: (*w).trim_matches(|p| p == ',').into(),
                weight: 0,
                childs: vec![],
            },
        ))
    }
    if res.iter().any(|x| x.is_none()) {
        None
    } else {
        Some(res.iter().map(|x| x.unwrap()).collect())
    }
}

fn main() {
    let mut f = File::open(Path::new("input/day7.txt")).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();
    let tower = read_input(&input);
    println!("Part 1: {}", tower.name)
}

fn read_input(file: &str) -> Tower {
    let mut building_towers = vec![];
    let mut finished_towers = vec![];
    // create all the towers
    for line in file.lines() {
        let s = line;
        let (t, finished) = new_tower(&s);
        if finished {
            finished_towers.push(t);
        } else {
            building_towers.push(s);
        }
    }
    // create childs
    let mut i = 0;
    while building_towers.len() > 0 {
        i = i % building_towers.len();
        match find_children(&building_towers[i], &finished_towers) {
            Some(mut c) => {
                let mut remove_counter = 0;
                let (mut t, _) = new_tower(&building_towers[i]);
                c.sort();
                for j in c {
                    t.insert(finished_towers.remove(j - remove_counter));
                    remove_counter += 1;
                }
                finished_towers.push(t);
                building_towers.remove(i);
            }
            None => i += 1,
        }
    }
    finished_towers.remove(0)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let t = Tower {
            name: "a".into(),
            weight: 0,
            childs: vec![],
        };
        let mut u = Tower {
            name: "b".into(),
            weight: 1,
            childs: vec![],
        };
        u.insert(t);
        assert_eq!(u.childs.first().unwrap().name, "a")
    }

    #[test]
    fn test_read() {
        let mut input = "b 15\na 53 -> b c \nc 1";
        let mut u = read_input(input);
        assert_eq!(
            u.childs
                .iter()
                .map(|x| x.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["b", "c"]
        );
        input = "pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)";
        u = read_input(input);
        assert_eq!(u.name.as_str(), "tknk")
    }

}
