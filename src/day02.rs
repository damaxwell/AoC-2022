use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{anyhow,Result,Context,Error};
use std::mem::transmute;

#[derive(PartialEq,Copy,Clone,Debug)]
#[repr(i64)]
enum Weapon {
  Rock,
  Paper,
  Scissors,
}
impl Weapon {
  fn subordinate(self) -> Weapon {
    unsafe { transmute( ((self as i64) - 1).rem_euclid(3) ) }
  }
  fn superior(self) -> Weapon {
    unsafe { transmute( ((self as i64) + 1) % 3) }
  }

  fn shape_score(self) -> i64 {
    (self as i64) + 1
  }

}
impl TryFrom<char> for Weapon {
  type Error = Error;
  fn try_from(c: char) -> Result<Self, anyhow::Error> {
    match c {
      'A' | 'X' => Ok( Weapon::Rock ),
      'B' | 'Y' => Ok( Weapon::Paper ),
      'C' | 'Z' => Ok( Weapon::Scissors ),
      _ => Err(anyhow!("Unknown weapon {}",c))
    }
  }
}


fn game_score(a: Weapon, b: Weapon) -> i64 {

  let score = 
    if a == b {
      3
    } else if a == b.subordinate() {
      6
    } else {
      0
    };

  score + b.shape_score()
}

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );


  let mut strategy = Vec::new();
  for line in r.lines() {
    let line = line?;
    let mut c = line.chars();
    let op = Weapon::try_from(c.next().unwrap())?;
    c.next().ok_or( anyhow!("unexpected EOF") )?;
    let me = Weapon::try_from(c.next().unwrap())?;

    strategy.push( (op,me) );
  }

  let mut total_score = 0;
  for (a,b) in &strategy {
    total_score += game_score(*a,*b);
  }

  let mut star2 = 0;
  for (a,b) in &strategy {
    let score = match b {
      // 'X' => loss, not rock
      Weapon::Rock => a.subordinate().shape_score(),
      // 'Y' => tie, not paper
      Weapon::Paper => 3 + a.shape_score(),
      // 'Z' => win, not scissors
      Weapon::Scissors => 6 + a.superior().shape_score()
    };
    star2 += score;
  }

  let star1 = total_score;

  Ok( Solution::from_i64( star1, star2 ) )  
}
