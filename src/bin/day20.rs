use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::ops::{Add, Sub, Div};

#[derive(Debug, PartialEq, Clone)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Particle {
    fn update(&mut self) {
        self.v = &self.v + &self.a;
        self.p = &self.p + &self.v;
    }

    fn time_to_reverse(&self) -> isize {
        let v = &self.v / &self.a;
        -v.min_coord()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn manhattan_dist(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn min_coord(&self) -> isize {
        self.x.min(self.y).min(self.z)
    }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;
    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;
    fn sub(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Div<&'b Vector> for &'a Vector {
    type Output = Vector;
    fn div(self, other: &'b Vector) -> Vector {
        Vector {
            x: if other.x != 0 { self.x / other.x } else { 0 },
            y: if other.y != 0 { self.x / other.y } else { 0 },
            z: if other.z != 0 { self.x / other.z } else { 0 },
        }
    }
}

fn parse_vector(input: &str) -> Option<Vector> {
    let i = input
        .split(",")
        .filter_map(|x| x.trim().parse::<isize>().ok())
        .collect::<Vec<isize>>();
    if i.len() == 3 {
        Some(Vector {
            x: i[0],
            y: i[1],
            z: i[2],
        })
    } else {
        None
    }
}

fn parse_particle(input: &str) -> Particle {
    let mut parsed = input.split(|c| c == '<' || c == '>').filter_map(
        |x| parse_vector(x),
    );
    Particle {
        p: parsed.next().unwrap(),
        v: parsed.next().unwrap(),
        a: parsed.next().unwrap(),
    }
}

fn solve1(particles: &[Particle]) -> usize {
    // find the minimum acceleration
    let mut min = particles[0].a.manhattan_dist();
    let mut min_index = 0;
    for (i, part) in particles.iter().enumerate() {
        let acc = part.a.manhattan_dist();
        if acc < min {
            min = acc;
            min_index = i;
        }
    }
    min_index
}

fn run(particles: &mut Vec<Option<Particle>>) {
    let mut positions: Vec<Vector> = vec![];
    let mut pos_collisions: Vec<Vector> = vec![];
    // Detect collisions and update positions
    for x in particles.iter_mut().filter(|p| p.is_some()) {
        if let &mut Some(ref mut p) = x {
            p.update();
            if positions.iter().any(|pos| *pos == p.p) {
                pos_collisions.push(p.p.clone());
            } else {
                positions.push(p.p.clone());
            }
        }
    }
    for x in particles.iter_mut().filter(|p| p.is_some()) {
        if pos_collisions.iter().any(|p| *p == x.as_ref().unwrap().p) {
            *x = None;
        }
    }
}

fn wont_collide(p1: &Particle, p2: &Particle) -> bool {
    if p1 == p2 {
        true
    } else {
        let dx = &p1.p - &p2.p;
        let dv = &p2.v - &p1.v;
        dx.x * dv.x < 0 || dx.y * dv.y < 0 || dx.z * dv.z < 0
    }
}

fn solve2(particles: &mut Vec<Option<Particle>>) -> usize {
    // time for each part to go in the direction of their acceleration and aaand a bit more time
    let time = particles.iter().fold(0, |time, particle| {
        time.max(particle.as_ref().unwrap().time_to_reverse())
    }) + 100;
    for _ in 0..time {
        run(particles);
    }
    // Remove particles that will only go away from the others
    let mut away_removed = 0;
    while particles.iter().filter(|x| x.is_some()).count() > 0 {
        run(particles);
        // WIP: use wont_collide to remove particles that will never collide anymore
    }
    away_removed
}

fn solve3(particles: &mut Vec<Option<Particle>>) -> usize {
    for _ in 0..100 {
        run(particles);
    }
    particles.iter().filter(|p| p.is_some()).count()
}

fn main() {
    let mut f = File::open(Path::new("input/day20.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let mut particles = vec![];
    for line in s.lines() {
        particles.push(parse_particle(line));
    }
    // particles = vec![
    //     Particle{p: Vector{x: -6, y: 0, z: 0},
    //         v: Vector{x: 3, y: 0, z: 0},
    //         a: Vector{x: 0, y: 0, z: 0}},
    //     Particle{p: Vector{x: -4, y: 0, z: 0},
    //         v: Vector{x: 2, y: 0, z: 0},
    //         a: Vector{x: 0, y: 0, z: 0}},
    //     Particle{p: Vector{x: -2, y: 0, z: 0},
    //         v: Vector{x: 1, y: 0, z: 0},
    //         a: Vector{x: 0, y: 0, z: 0}},
    //     Particle{p: Vector{x: 3, y: 0, z: 0},
    //         v: Vector{x: -1, y: 0, z: 0},
    //         a: Vector{x: 0, y: 0, z: 0}},
    // ];
    let p1 = solve1(&particles);
    let p2 = solve3(&mut particles.iter().map(|p| Some(p.clone())).collect());
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn parse_test() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>";
        assert_eq!(
            parse_particle(input),
            Particle {
                p: Vector { x: 3, y: 0, z: 0 },
                v: Vector { x: 2, y: 0, z: 0 },
                a: Vector { x: -1, y: 0, z: 0 },
            }
        );
    }

    #[test]
    fn solve1_test() {
        let particles = [
            Particle {
                p: Vector { x: 3, y: 0, z: 0 },
                v: Vector { x: 2, y: 0, z: 0 },
                a: Vector { x: -1, y: 0, z: 0 },
            },
            Particle {
                p: Vector { x: 4, y: 0, z: 0 },
                v: Vector { x: 0, y: 0, z: 0 },
                a: Vector { x: -2, y: 0, z: 0 },
            },
        ];
        assert_eq!(solve1(&particles), 0);
    }

    #[test]
    fn solve2_test() {
        let mut particles = vec![
            Particle {
                p: Vector { x: -6, y: 0, z: 0 },
                v: Vector { x: 3, y: 0, z: 0 },
                a: Vector { x: 0, y: 0, z: 0 },
            },
            Particle {
                p: Vector { x: -4, y: 0, z: 0 },
                v: Vector { x: 2, y: 0, z: 0 },
                a: Vector { x: 0, y: 0, z: 0 },
            },
            Particle {
                p: Vector { x: -2, y: 0, z: 0 },
                v: Vector { x: 1, y: 0, z: 0 },
                a: Vector { x: 0, y: 0, z: 0 },
            },
            Particle {
                p: Vector { x: 3, y: 0, z: 0 },
                v: Vector { x: -1, y: 0, z: 0 },
                a: Vector { x: 0, y: 0, z: 0 },
            },
        ];
        assert_eq!(wont_collide(&particles[1], &particles[2]), false);
        assert_eq!(solve2(&mut particles), 1);
    }
}
