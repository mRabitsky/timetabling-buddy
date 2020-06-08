//! USAGE:
//!     timetabling-buddy [OPTIONS] <course-list>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -n, --course-load <course-load>...    Either a single number for the total courses you want to take, or a pair of numbers for the max number of courses per term [default: 10]
//!     -m, --must <must>...                  Courses that the schedule must contain
//!
//! ARGS:
//!     <course-list>    JSON file containing a list of course sections

use itertools::Itertools;
use timetabling_buddy::TimetablingBuddy;

fn main() {
    let schedules = TimetablingBuddy::new().get();
    println!("Possible schedules ({}):\n{}", schedules.len(), schedules.join("\n"));
}

