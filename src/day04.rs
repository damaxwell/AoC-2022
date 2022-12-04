use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{anyhow,Result,Context};

struct IPair {
  a: i64,
  b: i64
}
impl IPair {
  fn contains(&self, other: &IPair) -> bool {
    (self.a <= other.a) && (self.b >= other.b)
  }

  fn overlaps(&self, other: &IPair) -> bool {
    (self.a <= other.b) && (self.b >= other.a)
  }

}


pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );


  let mut assignments: Vec<(IPair,IPair)> = Vec::new();
  for line in r.lines() {
    let line = line.with_context(|| format!("reading file: {}", &input_path) )?;
    
    let mut endpoints = line.split(&[',','-']);
    let task1 = IPair { a:endpoints.next()
                                   .ok_or_else(|| anyhow!("Missing first task start"))
                                   .with_context(|| format!("parsing line: '{}'",line))?
                                   .parse()
                                   .with_context(|| format!("parsing line: '{}'",line))?,
                        b:endpoints.next()
                                   .ok_or_else(|| anyhow!("Missing first task end"))
                                   .with_context(|| format!("parsing line: '{}'",line))?
                                   .parse()
                                   .with_context(|| format!("parsing line: '{}'",line))? };
    let task2 = IPair { a:endpoints.next()
                                   .ok_or_else(|| anyhow!("Missing second task start"))
                                   .with_context(|| format!("parsing line: '{}'",line))?
                                   .parse()
                                   .with_context(|| format!("parsing line: '{}'",line))?,
                        b:endpoints.next()
                                   .ok_or_else(|| anyhow!("Missing second task end"))
                                   .with_context(|| format!("parsing line: '{}'",line))?
                                   .parse()
                                   .with_context(|| format!("parsing line: '{}'",line))? };

     assignments.push( (task1,task2) );
  }

  let mut star1 = 0;
  let mut star2 = 0;

  for (p1, p2) in &assignments {
    if p1.contains(&p2) || p2.contains(&p1) {
      star1 += 1;
    }
  }

  for (p1, p2) in &assignments {
    if p1.overlaps(&p2)  {
      star2 += 1;
    }
  }

  Ok( Solution::from_i64( star1, star2 ) )  
}
