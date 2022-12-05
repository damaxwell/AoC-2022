use crate::Solution;
use crate::AppArgs;
use std::fs::{File};
use std::io::{BufReader,Read};
use anyhow::{anyhow,Result,Context};

struct IPair {
  a: u64,
  b: u64
}
impl IPair {
  fn new(a: u64, b: u64) -> Self {
    IPair{ a: a, b: b}
  }

  fn contains(&self, other: &IPair) -> bool {
    (self.a <= other.a) && (self.b >= other.b)
  }

  fn overlaps(&self, other: &IPair) -> bool {
    (self.a <= other.b) && (self.b >= other.a)
  }

}

struct Scanner {
  buf: Vec<u8>,
  p: usize
}
impl Scanner {
  fn new( buf: Vec<u8> ) -> Self {
    Scanner {
      buf: buf,
      p: 0
    }
  }

  fn next_u64(&mut self) -> Option<u64> {
    let mut q = self.p;
    let mut val = 0;
    while q < self.buf.len() && self.buf[q].is_ascii_digit() {
      val *= 10;
      val += u64::from(self.buf[q] - b'0');
      q += 1;
    }
    if self.p < q {
      self.p = q;
      Some( val )
    } else {
      None
    }
  }

  fn next_i64(&mut self) -> Option<i64> {
    let mut q = self.p;
    let mut sign = 1;
    let mut val = 0;

    while q < self.buf.len() && self.buf[q] == b'-' {
      sign *= -1;
      q += 1;
    }

    let dstart = q;
    while q < self.buf.len() && self.buf[q].is_ascii_digit() {
      val *= 10;
      val += i64::from(self.buf[q] - b'0');
      q += 1;
    }
    if dstart < q {
      self.p = q;
      Some( sign*val )
    } else {
      None
    }
  }

  fn is_eof(&self) -> bool {
    self.p >= self.buf.len()
  }

  fn expect(&mut self, b: u8) -> Option<u8> {
      if self.p < self.buf.len() && self.buf[self.p] == b {
        self.p += 1;
        Some(b)
      } else {
        None
      }
  }

}

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let input_path = args.input_file_path();
  let mut r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );

  let mut buf = Vec::new();
  r.read_to_end(&mut buf)?;
  let mut scan = Scanner::new( buf );

  let mut star1 = 0;
  let mut star2 = 0;

  while !scan.is_eof() {

    let p0 = scan.next_i64().unwrap();
    scan.expect(b'-').unwrap();
    let p1 = scan.next_i64().unwrap();

    scan.expect(b',').unwrap();

    let q0 = scan.next_i64().unwrap();
    scan.expect(b'-').unwrap();
    let q1 = scan.next_i64().unwrap();

    if scan.expect(b'\n').is_none() {
      if !scan.is_eof() {
        return Err(anyhow!("Unexected EOF"));
      }
    }

    println!("{} {} {} {}",p0,p1,q0,q1);
    let task1 = IPair::new(p0 as u64, p1 as u64);
    let task2 = IPair::new(q0 as u64, q1 as u64);

    if task1.contains(&task2) || task2.contains(&task1)
    {
      star1 += 1;
      star2 += 1;
    } else if task1.overlaps(&task2) {
      star2 += 1;
    }
  }

  Ok( Solution::from_i64( star1, star2 ) )  
}
