extern crate structopt;

use crate::structopt::StructOpt;

struct World {
    current: Vec<bool>,
    size: usize,
}

impl World {
    pub fn new(size: usize) -> World {
        let mut current: Vec<bool> = [false].iter().cycle().map(|x| *x).take(size).collect();
        current[size / 2] = true;
        World {
            current: current,
            size: size,
        }
    }

    pub fn display(&self) {
        self.current
            .iter()
            .map(|item| if *item { "*" } else { " " })
            .for_each(|x| print!("{}", x));
        println!("");
    }

    fn get_around(&self, idx: usize) -> Option<(bool, bool, bool)> {
        if self.size <= idx {
            return None;
        }

        match idx {
            0 => Some((false, self.current[0], self.current[1])),
            v if self.current.len() - 1 == v => Some((self.current[v - 1], self.current[v], false)),
            _ => Some((
                self.current[idx - 1],
                self.current[idx],
                self.current[idx + 1],
            )),
        }
    }

    pub fn next(&self, rule: &impl Fn((bool, bool, bool)) -> bool) -> World {
        World {
            current: (0..self.current.len())
                .map(|idx| rule(self.get_around(idx).unwrap()))
                .collect(),
            size: self.size,
        }
    }
}

fn rule(no: u8) -> impl Fn((bool, bool, bool)) -> bool {
    move |bits: (bool, bool, bool)| match bits {
        (false, false, false) => 0x01 & no != 0,
        (false, false, true) => (0x01 << 1) & no != 0,
        (false, true, false) => (0x01 << 2) & no != 0,
        (false, true, true) => (0x01 << 3) & no != 0,
        (true, false, false) => (0x01 << 4) & no != 0,
        (true, false, true) => (0x01 << 5) & no != 0,
        (true, true, false) => (0x01 << 6) & no != 0,
        (true, true, true) => (0x01 << 7) & no != 0,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "odcel")]
struct Options {
    /// rule number 0-255
    #[structopt(name = "RULE")]
    rule_number: u8,

    /// num of generation
    #[structopt(name = "GENERATION")]
    generation: usize,
}

fn main() {
    let opt = Options::from_args();

    let mut world = World::new(opt.generation * 2);
    let r = rule(opt.rule_number);
    for _ in 0..opt.generation {
        world.display();
        world = world.next(&r);
    }
}
