mod day01;
mod day02;
mod day03;
mod day03b;

use std::fmt;
use std::time::Instant;
use anyhow::{anyhow,Result};

use lazy_static::lazy_static;
use std::collections::HashMap;

type Solver = fn(&AppArgs) -> Result<Solution>;
lazy_static! {
  static ref SOLVERS: HashMap<&'static str, Solver> = {
    let mut m = HashMap::new();
    m.insert("01",day01::solve as Solver);
    m.insert("02",day02::solve as Solver);
    m.insert("03",day03::solve as Solver);
    m.insert("03b",day03b::solve as Solver);
    m
  };
}

pub struct Solution {
    part_a: i64,
    part_b: Option<i64>
}

impl Solution {
  pub fn from_i64( part_a: i64, part_b: i64 ) -> Self {
    Solution {
      part_a: part_a,
      part_b: Some( part_b )
    }
  }

  pub fn from_usize( part_a: usize, part_b: usize ) -> Result<Self> {
    Ok( Solution {
      part_a: part_a.try_into()?,
      part_b: Some( part_b.try_into()? ) 
    } )
  }

}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solution:\n")?;
        write!(f, "  Part a: {}\n", self.part_a)?;
        match self.part_b {
            Some(b) => write!(f, "  Part b: {}", b),
            _ => write!(f,"  Part b: not completed")
        }
    }
}

pub struct AppArgs {
    debug_mode: bool,
    help_mode: bool,
    data_file_path: Option<String>,
    day: String
}

impl AppArgs{
    fn parse<T>( args: T) -> Result<AppArgs> 
      where T: Iterator<Item=String> {
        let mut day = None;
        let mut help_mode = false;
        let mut debug_mode = false;
        let mut data_file_path = None;
        let mut args = args.into_iter();
        
        // Skip the executable name.
        args.next();

        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                if arg == "-d" || arg == "--debug" {
                    debug_mode = true;
                    continue;
                }
                if arg == "-f" || arg == "--file" {
                    data_file_path = match args.next() {
                        Some(f) => Some(f),
                        _ => return Err(anyhow!("Missing file path argument for -f/--file"))
                    };
                    continue;
                }
                if arg == "-h" || arg == "--help" {
                    help_mode = true;
                    continue;
                }
                return Err(anyhow!("Unknown argument {}",arg))
            }

            // This had better be the last argument
            if let Some(arg2) = args.next() {
                return Err(anyhow!("Unknown extra argument {}", arg2));
            }

            // A valid day has the form of one or two digits followed
            // by an optional suffix.  E.g. 1b, 18, 22-slow
            // For just one digit we prepend a zero for the official name.

            let mut prefix = arg.chars();
            day = if prefix.next().map_or(false, |c| c.is_ascii_digit() ) &&
                    !prefix.next().map_or(false, |c| c.is_ascii_digit() ) {
                let mut rv = "0".to_owned();
                rv.push_str( &arg );
                Some( rv )
            } else {
                Some( arg )
            }
        }

        let day = day.ok_or_else( || anyhow!("Missing day argument") )?;

        Ok( AppArgs { help_mode: help_mode, 
                      debug_mode: debug_mode, 
                      data_file_path: data_file_path, 
                      day: day })
    }

    pub fn input_file_path(&self) -> String {

        let debug_suffix = if self.debug_mode {
            "-d"
        } else {
            ""
        };

        self.data_file_path.as_ref().map( |s| s.clone() ).unwrap_or_else( || 
            format!("data/day{}{}.txt",self.day,debug_suffix) 
        )
    }
}

impl fmt::Display for AppArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppArgs:\n")?;
        write!(f, "  Day: {}\n", self.day)?;
        match self.data_file_path {
            Some(ref p) => {
                write!(f, "  Data file path: {}\n", p)?;
            }
            None => {
                write!(f, "  Data file path: DEFAULT")?;                
            }
        }
        write!(f, "  Debug mode: {}\n", self.debug_mode)
    }    
}


fn usage() {
    let app_name:String = std::env::args().next().unwrap_or_else( || String::from(""));

    println!("usage:");
    println!("    {} [-h,--help] [-d,--debug] [-f/--file PATH] DAY", app_name);
    println!("");
    println!("Input file is inferred if not given by --file:");
    println!("  data/dayXX.txt or");
    println!("  data/dayXX-d.txt in debug mode");
}

fn main() -> Result<()> {

    let args = AppArgs::parse( std::env::args() ).map_err( |e| {usage(); e} )?;

    if args.help_mode {
        usage();
        return Ok(());
    }

    let d: &str = args.day.as_ref();
    let solver = SOLVERS.get(d)
                        .ok_or_else(|| anyhow!("No solver available for day {}", args.day))?;

    let now = Instant::now();
    let solution = solver(&args)?;
    let elapsed_time = now.elapsed();

    println!("Elapsed time: {} μs", elapsed_time.as_micros());
    println!("{}",solution);

    Ok(())
}
