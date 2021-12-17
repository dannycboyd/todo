// TODO: Delete this stuff

use chrono::NaiveDateTime;

#[derive(Debug)]
pub enum Mod {
  // used by lalrpop parsing
  Start(Option<NaiveDateTime>),
  End(Option<NaiveDateTime>),
  Rep(String),
  Title(String),
  Note(String),
  Cal(bool),
  Todo(bool),
  Journal(bool)
}

#[derive(Debug)]
pub struct Mods {
  pub data: Vec<Mod>
}

impl Mods {
  pub fn new() -> Self {
    Self { data: vec![] }
  }

  pub fn has_changes(&self) -> bool {
    self.data.len() > 0
  }

  pub fn push(&mut self, mod_item: Mod) {
    self.data.push(mod_item);
  }

  // could simplify some things to add an iter implementation
}

#[derive(Debug)]
pub struct RawTaskItem {
  pub start: Vec<u32>,
  pub repetition: String,
  pub title: String,
  pub note: String,
  pub finished: bool
}

impl RawTaskItem {
  pub fn new_empty() -> RawTaskItem {
    RawTaskItem {
      start: vec![],
      repetition: String::from("m"),
      title: String::from("Title"),
      note: String::from(""),
      finished: false
    }
  }
}
