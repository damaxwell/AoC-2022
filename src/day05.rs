use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader};
use std::io::{BufRead};
use anyhow::{Result,Context};

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut stacks = Vec::new();
  for _ in 0..9 {
    stacks.push( Vec::new());
  }

  let mut lines = r.lines();
  loop {
    let line = lines.next().unwrap()?;
    if line.starts_with(" 1 ") {
      break;
    }

    for (k,c) in line.chars().enumerate() {
      if c.is_ascii_alphabetic() {
        let col = (k-1) / 4;
        stacks[col].push(c);
      }
    }
  }
  lines.next();

  stacks.iter_mut().for_each(|s| s.reverse() );
  let mut stacks_2 = stacks.clone();
  let mut tmp = Vec::new();

  for l in lines {
    let l = l?;

    let mut words = l.split(" ");
    words.next();
    let amount = words.next().unwrap().parse::<usize>()?;
    words.next();
    let from = words.next().unwrap().parse::<usize>()?;
    words.next();
    let to = words.next().unwrap().parse::<usize>()?;

    for _ in 0..amount {
      let c = stacks[from-1].pop().unwrap();
      stacks[to-1].push(c);
    }

    for _ in 0..amount {
      let c = stacks_2[from-1].pop().unwrap();
      tmp.push(c);
    }
    for _ in 0..amount {
      stacks_2[to-1].push( tmp.pop().unwrap() );
    }

  }

  stacks.iter().for_each(|s| print!("{}", *s.last().unwrap() ) );
  println!("");

  stacks_2.iter().for_each(|s| print!("{}", *s.last().unwrap() ) );
  println!("");

  let star1 = 0;
  let star2 = 0;

  Ok( Solution::from_i64( star1, star2 ) )  
}
