# timetabling-buddy
## About
Script to find all possible timetables for UBC courses given certain constraints.

Takes in a single `.json` file and outputs all possible schedules.  An example file is provided [here](exampleCourseList.json).

## Installation
### Pre-built binaries
There are a few pre-built binaries available here on GitHub.
### Building from source
You will need a Rust installation to build this project.  If you do not already have Rust installed, [click here](https://rustup.rs/).
With Rust installed, you can either download the source directly, or clone in using `git`.
```bash
$ git clone https://github.com/mRabitsky/timetabling-buddy.git
$ cargo build --release
```

## Usage
    timetabling-buddy [OPTIONS] <course-list>
####FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
####OPTIONS:
    -n, --course-load <course-load>...    Either a single number for the total courses you want to take, or a pair of numbers for the max number of courses per term [default: 10]
    -m, --must <must>...                  Courses that the schedule must contain
####ARGS:
    <course-list>    JSON file containing a list of course sections

## *Caveat emptor*
This project is mostly untested.  Use at your own risk.  I don't make any guarantees that the output will be correct or that the script will even work.  If you do find a bug though, feel free to report it here.