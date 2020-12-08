use crate::Opcode::{ACC, JMP, NOP};

fn main() {
    const INPUT: &str = include_str!("input");
    let mut vec = INPUT.lines().map(Ins::from).collect::<Vec<Ins>>();

    println!("Answer 1: {:?}", vec.run());

    vec.reset();

    let mut c = 0;
    let code = loop {
        let mut vec = vec.clone();
        let ins = &mut vec[c];
        match ins.opcode {
            ACC => { c += 1; continue },
            JMP => ins.opcode = NOP,
            NOP => ins.opcode = JMP
        };

        if let Ok(exit_code) = vec.run() {
            break exit_code;
        } else {
            c += 1;
        }
    };
    println!("Answer 2: {}", code);
}

trait Process {
    fn run(&mut self) -> Result<isize, isize>;
    fn reset(&mut self);
}

impl Process for Vec<Ins> {
    fn run(&mut self) -> Result<isize, isize> {
        let mut acc: isize = 0;
        let mut cur: isize = -1;

        loop {
            cur += 1;
            let ins = match self.get_mut(cur as usize) {
                Some(ins) => ins,
                None => {
                    break Ok(acc);
                }
            };
            if let Err(_) = ins.run_panic(&mut acc, &mut cur) {
                break Err(acc);
            }
        }
    }

    fn reset(&mut self) {
        for x in self {
            x.visited = false;
        }
    }
}

impl Ins {
    fn new(opcode: Opcode, operand: isize) -> Self {
        Ins {
            opcode,
            operand,
            visited: false
        }
    }

    #[inline]
    fn run_panic(&mut self, acc: &mut isize, cur: &mut isize) -> Result<(), ()> {
        if self.visited {
            Err(())
        } else {
            Ok(self.run(acc, cur))
        }
    }

    #[inline]
    fn run(&mut self, acc: &mut isize, cur: &mut isize) {
        match self.opcode {
            JMP => *cur += self.operand - 1,
            ACC => *acc += self.operand,
            NOP => {}
        };
        self.visited = true;
    }
}

impl From<&str> for Ins {
    fn from(s: &str) -> Self {
        let (code, operand) = s.split_at(3);
        let code = match code {
            "acc" => ACC,
            "jmp" => JMP,
            "nop" => NOP,
            c => panic!("invalid instruction {}", c)
        };
        let operand: isize = operand.trim().trim_start_matches("+").parse().unwrap();
        Self::new(code, operand)
    }
}

#[derive(Clone)]
struct Ins {
    opcode: Opcode,
    operand: isize,
    visited: bool
}

#[derive(Debug, Clone)]
enum Opcode {
    NOP,
    ACC,
    JMP
}