use crate::{AppArgs,Solution};
use std::fs::{File};
use std::io::{BufReader,BufRead};
use anyhow::{Result,Context};
use std::collections::HashMap;

enum FSEntry {
  Dir(usize),
  File(usize)
}

pub fn solve(args: &AppArgs) -> Result<Solution> {

  let mut star1 = 0;
  let mut star2 = 0;

  let input_path = args.input_file_path();
  let r = BufReader::new( File::open( &input_path )
                .with_context(|| format!("Opening file: {}", &input_path) )? );
  let mut lines = r.lines();


  let mut file_system: Vec<HashMap<String, FSEntry>> = Vec::new();
  file_system.push( HashMap::new() );

  let mut path = Vec::new();
  let mut cwd = 0;
  path.push(cwd);

  let mut curr_line = lines.next().unwrap()?;
  'outer: loop {
    if curr_line == "$ cd /" {
      let l = lines.next();
      if l.is_none() {
        break;
      }
      curr_line = l.unwrap()?;
      continue;
    }
    if curr_line.starts_with("$ ls") {

      let l = lines.next();
      if l.is_none() {
        break;
      }
      curr_line = l.unwrap()?;

      while !curr_line.starts_with("$") {
        let mut pair = curr_line.split(" ");

        let p0 = pair.next().unwrap();
        if p0 == "dir" {
          let name = String::from(pair.next().unwrap());

          let dir_id = file_system.len();
          file_system.push(HashMap::new());

          let new_dir = FSEntry::Dir(dir_id);
          file_system[cwd].insert(name, new_dir);

        } else {

          let size:usize = p0.parse()?;

          let new_file = FSEntry::File(size);

          let name = String::from(pair.next().unwrap());

          file_system[cwd].insert(name, new_file);
        }

        let l = lines.next();
        if l.is_none() {
          break 'outer;
        }
        curr_line = l.unwrap()?;
      }
    } else if curr_line.starts_with("$ cd ") {

      let new_dir = curr_line.split(" ").skip(2).next().unwrap();

      if new_dir == ".." {
        path.pop();
        cwd = *path.last().unwrap();
      } else {
        match file_system[cwd].get(new_dir).unwrap() {
          FSEntry::Dir(dir_id) => {
            cwd = *dir_id;
            if file_system.get(cwd).is_none() {}
            path.push(*dir_id);
          }
          _ => panic!()
        }
      }
      let l = lines.next();
      if l.is_none() {
        break;
      }
      curr_line = l.unwrap()?;
    } else {
      panic!();
    }

  }

  let mut sizes = vec![None; file_system.len()];
  sizes[0] = Some( compute_dir_size(&file_system, &mut sizes, 0) );

  for s in &sizes {
    let s = s.unwrap();
    if s <= 100000 {
      star1 += s as i64;
    }
  }

  let size_avail = 70000000 - sizes[0].unwrap();
  let size_needed = 30000000 - size_avail;

  sizes.sort();
  for s in sizes {
    let s = s.unwrap();
    if s > size_needed {
      star2 = s as i64;
      break;
    }
  }

  Ok( Solution::from_i64( star1, star2 ) )  
}

fn compute_dir_size(fs: &Vec<HashMap<String,FSEntry>>, sizes: &mut Vec<Option<usize>>, dir: usize) -> usize {
  let mut size = 0;
  for (_,entry) in &fs[dir] {
      match entry {
        FSEntry::Dir(d) => {
          let dsize = match sizes[*d] {
            Some(s) => s,
            _ => {
              let new_size =  compute_dir_size(fs,sizes,*d); 
              sizes[*d] = Some(new_size);
              new_size
            }
          };
          size += dsize;
        }
        FSEntry::File(s) => size += s
      }
  }
  size
}