use crate::{AppArgs,Solution};
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{Result,Context};
use std::fmt::{Display,Formatter};

enum Instr {
  NOp,
  Addv(i64)
}

const CRT_ROWS:usize = 6;
const CRT_COLS:usize = 40;

struct Crt {
  rows: [[char;CRT_COLS];CRT_ROWS],
  r: usize,
  c: usize
}
impl Crt {
  fn new() -> Self {
    Crt {
      rows: [[' ';CRT_COLS];CRT_ROWS],
      r: 0,
      c: 0
    }
  }

  fn update(&mut self, c: char) {
    self.rows[self.r][self.c] = c;

    self.c = (self.c + 1) % CRT_COLS;
    if self.c == 0 {
      self.r = (self.r + 1) % CRT_ROWS;
    }

  }
}

struct Cpu<'a> {
  reg: i64,
  pc: usize,
  cycle: usize,
  program: &'a Vec<Instr>,
  pending: Option<i64>
}
impl<'a> Cpu<'a> {
  fn new(program: &'a Vec<Instr>) -> Self {
    Cpu {
      reg: 1,
      cycle: 1,
      pc: 0,
      program: program,
      pending: None
    }
  }

  fn is_running(&self) -> bool {
    self.pc < self.program.len() || self.pending.is_some()
  }

  fn tick(&mut self) {
    if let Some(x) = self.pending {
      self.reg += x;
      self.pending = None;
    } else {
      match self.program[self.pc] {
        Instr::Addv(x) => self.pending=Some(x),
        Instr::NOp => (),
      }
      self.pc += 1;
    }
    self.cycle += 1;
   }
}

impl Display for Crt {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    for r in 0..CRT_ROWS {
      for c in 0..CRT_COLS {
        write!(formatter,"{}",self.rows[r][c])?;
      }
      write!(formatter,"\n")?;
    }
    Ok(())
  }
}


pub fn solve(args: &AppArgs) -> Result<Solution> {

  let mut star1 = 0;

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut instructions = Vec::new();
  for line in r.lines() {
    let line = line?;
    let mut toks = line.split(' ');
    match toks.next().expect("nonempty line") {
      "noop" => instructions.push(Instr::NOp),
      "addx" => instructions.push(Instr::Addv(toks.next().expect("number").parse::<i64>()? )),
      _ => panic!()
    }
  }

  let mut cpu = Cpu::new(&instructions);
  while cpu.is_running() {
    if (cpu.cycle + 20) % 40  == 0 {
      star1 += cpu.reg * i64::try_from(cpu.cycle)?;
    }
    cpu.tick();
  }
 
  let mut cpu = Cpu::new(&instructions);
  let mut crt = Crt::new();
  while cpu.is_running() {
    let col = crt.c as i64;
    let c = if cpu.reg - 1 <= col  && col <= cpu.reg + 1 {
      '█'
    } else {
      ' '
    };
    crt.update(c);
    cpu.tick();
  }

  println!("{}",crt);

  let star2 = 0;
  Ok( Solution::from_i64( star1, star2 ) )  
}
