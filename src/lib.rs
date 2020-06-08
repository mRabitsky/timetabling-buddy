extern crate smallvec;
#[macro_use] extern crate structopt;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use chrono::{NaiveTime, Weekday};
use itertools::Itertools;
use serde::Deserialize;
use smallvec::SmallVec;
use structopt::StructOpt;
use rayon::prelude::*;

#[derive(Clone, Debug, Deserialize)]
struct Course {
    dept: String,
    num: String,
    sections: SmallVec<[Section; 4]>,
    #[serde(default)] prereqs: SmallVec<[String; 2]>
} impl Course {
    pub fn code(&self) -> String {
        format!("{}{}", self.dept, self.num)
    }
}
impl Display for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} [{}]", self.dept, self.num, self.sections.iter().join(", "))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Section {
    #[serde(skip)] course: String,
    num: String,
    term: u8,
    times: SmallVec<[MeetingTime; 3]>
} impl Section {
    pub fn conflicts_with(&self, other: &Section) -> bool {
        self.term == other.term && self.times.iter().any(|t: &MeetingTime| other.times.iter().any(|tt| t.intersects(tt)))
    }
}
impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}
impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.course == other.course && self.num == other.num
    }
}

#[derive(Clone, Debug, Deserialize)]
struct MeetingTime {
    day: Weekday,
    start: NaiveTime,
    end: NaiveTime
} impl MeetingTime {
    fn intersects(&self, other: &MeetingTime) -> bool {
        self.day == other.day && (self.end >= other.start && other.end >= self.start)
    }
}

pub struct TimetablingBuddy(Opt);
impl TimetablingBuddy {
    pub fn new() -> TimetablingBuddy {
        TimetablingBuddy(Opt::from_args())
    }
    pub fn get(&self) -> Vec<String> {
        let mut course_list: Vec<Course> = serde_json::from_reader(BufReader::new(File::open(&self.0.course_list).expect("Coudln't open file..."))).unwrap();
        course_list.iter_mut().for_each(|c| {
            let code = c.code();
            c.sections.iter_mut().for_each(|s| s.course = code.clone());
        });
        course_list.iter()
            .combinations(self.0.course_load.iter().sum::<u8>() as usize)
            .filter(|c| c.iter().all(|cc| cc.prereqs.iter().all(|p| c.iter().any(|ccc| ccc.code().eq_ignore_ascii_case(p)))))
            .par_bridge()
            .flat_map(|c| self.get_timetables_for_combo(c))
            .map(|s| format!(
                "[{}]",
                s.into_iter().map(|ss| format!("{} {}", ss.course, ss.num)).join(", ")
            ))
            .collect()
    }

    fn get_timetables_for_combo<'a>(&self, c: Vec<&'a Course>) -> Vec<Vec<&'a Section>> {
        let clone = c.clone();
        c.into_iter()
            .map(|cc| cc.sections.iter())
            .multi_cartesian_product()
            .par_bridge()
            .filter(|cc| self.timetable_validator(cc, &clone))
            .collect()
    }

    fn timetable_validator(&self, tt: &Vec<&Section>, cs: &Vec<&Course>) -> bool {
        cs.par_iter()
            .filter(|c| c.prereqs.len() > 0)
            .all(|c| c.prereqs.iter()
                .all(|p| tt.iter().find(|cc| cc.course == c.code()).unwrap().term > tt.iter().find(|cc| &cc.course == p).unwrap().term))
        && !tt.par_iter().any(|a| tt.iter().any(|b| a != b && a.conflicts_with(b)))
        && (if self.0.course_load.len() > 1 {
            tt.iter().filter(|c| c.term == 1).count() == self.0.course_load[0] as usize
            && tt.iter().filter(|c| c.term == 2).count() == self.0.course_load[1] as usize
        } else { true })
        && self.0.must.as_ref().map_or(true, |v| v.par_iter().all(|c| tt.iter().any(|cc| cc.course.eq_ignore_ascii_case(c))))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(author, rename_all = "kebab-case")]
pub struct Opt {
    /// JSON file containing a list of course sections
    #[structopt(parse(from_os_str), empty_values = false, validator_os = file_exists)]
    course_list: PathBuf,

    /// Either a single number for the total courses you want to take, or a pair of numbers for the max number of courses per term
    #[structopt(short = "n", long, min_values = 1, max_values = 2, default_value = "10")]
    course_load: Vec<u8>,

    /// Courses that the schedule must contain
    #[structopt(short, long)]
    must: Option<Vec<String>>
}
fn file_exists(file: &OsStr) -> Result<(), OsString> {
    let path = Path::new(&file);
    if path.exists() {
        if path.extension().is_none() {
            Err(OsString::from("Could not read the file extension"))
        } else if path.extension().unwrap() != "json" {
            Err(OsString::from("Course lists must be JSON files"))
        } else {
            Ok(())
        }
    } else {
        Err(OsString::from("Could not find a course list at the path entered"))
    }
}