use crate::cal;
use crate::TDError;

#[derive(Debug)]
pub enum Mod { // used by lalrpop parsing
    Start(Vec<u32>),
    Rep(cal::Repetition),
    Title(String),
    Note(String),
}

impl Mod {
    pub fn to_sql(&self) -> Result<String, TDError> {
        Ok(match self {
            Self::Start(date_raw) => {
                let start = cal::get_start(&date_raw.to_vec())?;
                let start = start.format("%Y-%m-%d").to_string();
                format!("start='{}' ", start)
            },
            Self::Rep(r) => format!("repeats='{}' ", r.to_sql_string()),
            Self::Title(t) => format!("title='{}' ", t),
            Self::Note(n) => format!("note='{}' ", n)
        })
    }
}

#[derive(Debug)]
pub struct RawTaskItem {
    pub start: Vec<u32>,
    pub repetition: String,
    pub title: String,
    pub note: String,
    pub finished: bool,
}

impl RawTaskItem {
    pub fn new_empty() -> RawTaskItem {
        RawTaskItem {
            start: vec![],
            repetition: String::from("m"),
            title: String::from("Title"),
            note: String::from(""),
            finished: false,
        }
    }
}
