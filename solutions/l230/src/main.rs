use std::io;
use std::str::FromStr;

#[allow(dead_code)]
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    buffer
}

#[allow(dead_code)]
fn read<T: FromStr>() -> Result<T, T::Err> {
    read_line().trim().parse::<T>()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Result<Vec<T>, T::Err> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse::<T>())
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd)]
struct Actor {
    strength: i32,
    increase: i32,
    life: i32,
}

impl Actor {
    pub fn increase_strength(&mut self, other: &Self) {
        self.strength += other.increase;
    }

    pub fn fight(&self, other: &Self) -> bool {
        self.strength > other.strength
    }

    pub fn new(data: &[i32]) -> Self {
        Self {
            strength: data[0],
            increase: data[1],
            life: 1,
        }
    }
}

fn main() {
    let init_input = read_vec::<i32>().unwrap();
    let mut dragons: Vec<Actor> = Vec::new();

    for x in 0..init_input[1] {
        let d_input = read_vec::<i32>().unwrap();
        let dragon = Actor::new(d_input.as_slice());
        dragons.push(dragon);
    }

    let mut kirito = Actor::new(init_input.as_slice());

    dragons.sort_unstable_by(|n, p| n.partial_cmp(p).unwrap());
    for dragon in dragons {
        if kirito.fight(&dragon) {
            kirito.increase_strength(&dragon);
        } else {
            kirito.life = 0;
            break;
        }
    }

    if kirito.life == 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}
