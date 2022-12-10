use crate::{AppArgs,Solution};
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{Result,Context};
use std::collections::HashSet;

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut instructions = Vec::new();
  for line in r.lines() {
    let line = line?;
    let mut l = line.split(' ');
    let direction = l.next().unwrap();
    let direction  = match direction {
      "R" => (1,0),
      "L" => (-1,0),
      "U" => (0,1),
      "D" => (0,-1),
      _ => panic!()
    };

    let size = l.next().unwrap().parse::<usize>()?;
    instructions.push( (direction, size));
  }

  let mut head:(i64,i64) = (0,0);
  let mut tail:(i64,i64) = (0,0);

  let mut history = HashSet::new();
  history.insert(tail);

  for ((dx,dy),ell) in &instructions {
    for _ in 0..*ell {
      head = (head.0+dx,head.1+dy);
      tail = update_tail(&head,&tail);
      history.insert(tail);
    }
  }

  let mut history2 = HashSet::new();
  let mut chain = Vec::new();
  for _ in 0..10 {
    chain.push((0,0));
  }
  history2.insert(chain[9]);

  for ((dx,dy),ell) in &instructions {
    for _ in 0..*ell {
      head = chain[0];      
      head = (head.0+dx,head.1+dy);
      chain[0] = head;

      for k in 1..10 {
        chain[k] = update_tail(&chain[k-1],&chain[k]);
      }

      history2.insert(chain[9]);
    }
  }

  let star1 = history.len() as i64;
  let star2 = history2.len() as i64;

  Ok( Solution::from_i64( star1, star2 ) )  
}

fn update_tail(head: &(i64,i64), tail: &(i64,i64)) -> (i64,i64) {
  if (head.0-tail.0).abs() == 2 || (head.1-tail.1).abs() == 2{
    let mut dx = 0;
    let mut dy = 0;

    if tail.1 < head.1 {
      dy = 1;
    } else if tail.1 > head.1 {
      dy = -1;
    }

    if tail.0 < head.0 {
      dx = 1;
    } else if tail.0 > head.0 {
      dx = -1;
    }

    return (tail.0 + dx, tail.1 + dy);
  }
  *tail
}