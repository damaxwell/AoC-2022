use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,Read};
use std::io::{BufRead};
use anyhow::{anyhow,Result,Context};

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let mut r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut stacks = Vec::new();
  let mut stacks_2 = Vec::new();
  for _ in 0..9 {
    stacks.push( Vec::<u8>::new());
    stacks_2.push( Vec::<u8>::new());
  }

  let mut lines = r.lines();
  loop {
    let line = lines.next().unwrap()?;
    if line.starts_with(" 1 ") {
      break;
    }

    let s = line.as_bytes();
    for (k,b) in s.iter().enumerate() {
      if b.is_ascii_alphabetic() {
        println!("col {}", char::from(*b));
        let col = (k-1) / 4;
        stacks[col].push(*b);
        stacks_2[col].push(*b);
      }
    }
  }

  for s in &mut stacks {
    s.reverse();
  }
  for s in &mut stacks_2 {
    s.reverse();
  }

  for k in 0..9 {
    println!("----");
    for c in stacks[k].iter() {
      println!("{}",char::from(*c));
    }
  }
  lines.next();

  let mut tmp = Vec::<u8>::new();
  for l in lines {
    let l = l?;
    println!("line {}",l);



    let mut words = l.split(" ");
    words.next();
    let amount = words.next().unwrap().parse::<usize>()?;
    words.next();
    let from = words.next().unwrap().parse::<usize>()?;
    words.next();
    let to = words.next().unwrap().parse::<usize>()?;


  for k in 0..9 {
    for c in stacks[k].iter() {
      println!("{}",char::from(*c));
    }
  }


    for k in 0..amount {
      let c = stacks[from-1].pop().unwrap();
      stacks[to-1].push(c);
    }

  for k in 0..9 {
    for c in stacks_2[k].iter() {
      println!("{}",char::from(*c));
    }
  }


    for k in 0..amount {
      let c = stacks_2[from-1].pop().unwrap();
      tmp.push(c);
    }
    for k in 0..amount {
      stacks_2[to-1].push( tmp.pop().unwrap() );
    }
  }

  for k in 0..9 {
    print!("{}", char::from(*stacks[k].last().unwrap()) );
  }
println!("");


  for k in 0..9 {
    print!("{}", char::from(*stacks_2[k].last().unwrap()) );
  }
println!("");

  let mut star1 = 0;
  let mut star2 = 0;

  Ok( Solution::from_i64( star1, star2 ) )  
}
