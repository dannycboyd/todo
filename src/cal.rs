// repetition is defined here
// add functions for displaying repetition inside range
pub mod calendar {
    use serde::{Serialize, Deserialize};
    use chrono::NaiveDate;
    use chrono::Local; // Utc, Local
    use chrono::Datelike;
    use std::fmt;


    use ansi_term::Style;
    use ansi_term::Color::{Yellow};

    use crate::task::TaskItem;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Repetition {
        Never,
        Daily,
        Weekly,
        Monthly,
        Yearly,
        // Custom(MultiDays)
    }

    impl fmt::Display for Repetition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let printable = match self {
                Repetition::Never => "Never",
                Repetition::Daily => "Daily",
                Repetition::Weekly => "Weekly",
                Repetition::Monthly => "Monthly",
                Repetition::Yearly => "Yearly",
                // Repetition::Custom(_custom) => "Custom"
            };
            write!(f, "{}", printable)
        }
    }

    // #[derive(Debug)]
    // pub struct MultiDays {
    //     sun: bool,
    //     mon: bool,
    //     tue: bool,
    //     wed: bool,
    //     thur: bool,
    //     fri: bool,
    //     sat: bool,
    // }

    // pub fn show_days_in_dur(tasks: Vec<&TaskItem>, start: NaiveDate, dur: u32) -> Vec<u32> {
    //     let mut days = vec![];
    //     for task in tasks.iter() {
    //         println!("{:?}", task);
    //         for i in 0..dur {
    //             match start.checked_add_signed(Duration::days(i as i64)) {
    //                 Some(check) => {
    //                     if task.occurs_on_day(check) { // this call zigzags, but i think it makes sense?
    //                         days.push(i);
    //                     }
    //                 },
    //                 None => println!("Something went wrong trying to add {} days to {:?}", i, &start),
    //             };
    //         };
    //     };
    //     println!("{:?}", days);
    //     days
    // }

    fn print_weekdays() {
        println!("Su Mo Tu We Th Fr Sa");
    }

    fn get_prev_sunday(date: NaiveDate) -> Option<NaiveDate> {
        let num_days = date.weekday().num_days_from_sunday();
        if num_days == 0 {
            Some(date)
        } else {
            let days: i32 = date.num_days_from_ce() - num_days as i32;
            NaiveDate::from_num_days_from_ce_opt(days)
        }
    }

    fn month_len(date: &NaiveDate) -> Option<i64> {
        next_month(date)
            .map(|end| end.signed_duration_since(NaiveDate::from_ymd(date.year(), date.month(), 1)).num_days())
    }

    fn next_month(date: &NaiveDate) -> Option<NaiveDate> {
        let mut year = date.year();
        let mut month = date.month();
        
        if month == 12 {
            year = year + 1;
            month = 1;
        } else {
            month = month + 1;
        }
        NaiveDate::from_ymd_opt(year, month, 1)
    }

    pub fn task_on_day(start: &NaiveDate, rep: &Repetition, check: NaiveDate) -> bool {
        match rep {
            Repetition::Never => start == &check,
            Repetition::Daily => start <= &check,
            Repetition::Weekly => start <= &check && start.weekday() == check.weekday(),
            Repetition::Monthly => start <= &check && start.day() == check.day(),
            _ => false
        }
    }

    pub fn show_type(kind: Repetition, start: NaiveDate, tasks: &Vec<TaskItem>) {
        match kind {
            Repetition::Never => (),
            Repetition::Daily => (),
            Repetition::Weekly => print_week(start, tasks),
            Repetition::Monthly => print_month(start, tasks),
            Repetition::Yearly => print_year(start, tasks),
        }
    }

    pub fn print_month(date: NaiveDate, tasks: &Vec<TaskItem>) {
        let year = date.year();
        let month = date.month();
        let current_month = NaiveDate::from_ymd(year, month, 1);
        let next_month = if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        };
        let duration = next_month.signed_duration_since(current_month).num_days();
        let date_str = current_month.format("%B %m|%Y").to_string();

        println!("\n{:^width$}", Style::new().bold().paint(&date_str), width=21);
        let mut first = current_month.weekday().num_days_from_sunday(); // number of days from sunday
        print_weekdays();
        if first < 7 {
            for _i in 0..first {
                print!("   ")
            }
        }
        for i in 1..=duration {
            first = (first + 1) % 7;
            let mut occurs = false;
            for t in tasks.iter() {
                if task_on_day(&t.start, &t.repetition, NaiveDate::from_ymd(year, month, i as u32)) {
                    occurs = true;
                    break;
                }
            };
            let day_justified = format!("{day:>2}", day=i);
            if occurs {
                print!("{}", Yellow.paint(day_justified));
            } else {
                print!("{}", day_justified);
            };
            if first == 0 {
                print!("\n");
            } else {
                print!(" ");
            }
        }
        println!("")
    }

    pub fn print_week(date: NaiveDate, tasks: &Vec<TaskItem>) {
        match get_prev_sunday(date) {
            None => println!("can't handle date: {:?}", date),
            Some(start) => {
                let mut happening: Vec<(u32, &TaskItem)> = vec![];
                let mut year = start.year();
                let mut month = start.month();
                let mut day: i64 = start.day() as i64;
                println!("Start: {:?}", start);
                match next_month(&start) {
                    None => println!("something went wrong checking the length of month: {:?}", start),
                    Some(next) => {
                        let length = next.signed_duration_since(NaiveDate::from_ymd(start.year(), start.month(), 1)).num_days();
                        print_weekdays();
                        for i in 0..7 {
                            if day + i == length + 1 {
                                year = next.year();
                                month = next.month();
                                day = 1 - i;
                            }
                            let mut occurs = false;
                            for t in tasks.iter() {
                                if task_on_day(&t.start, &t.repetition, NaiveDate::from_ymd(year, month, (day + i) as u32)) {
                                    happening.push(((day + i) as u32, t));
                                    occurs = true;
                                }
                            }

                            let day_justified = format!("{:>2} ", day+i);
                            let day_justified = if occurs { Yellow.bold().paint(day_justified).to_string() } else { String::from(day_justified) };
                            print!("{}", day_justified);
                        }
                        println!("");
                        let mut day: u32 = 0;
                        for (i, t) in happening.iter() {
                            if *i == day {
                                println!("    {}", t);
                            } else {
                                println!("{:>2}: {}", i, t.to_string());
                                day = *i;
                            }
                        }
                    }
                }
            }
        }
    }

    fn next_year(date: &NaiveDate) -> Option<NaiveDate> {
        NaiveDate::from_yo_opt(date.year() + 1, 1)
    }

    pub fn print_year(start: NaiveDate, tasks: &Vec<TaskItem>) {
        let year = start.year();
        match next_year(&start) {
            None => println!("Something went wrong trying to get the next year"),
            Some(next) => {
                let length: u32 = next.signed_duration_since(NaiveDate::from_yo(year, 1)).num_days() as u32;
                let width: usize = (length / 7) as usize;
                let mut days: Vec<String> = vec![String::new(); width];
                for i in 1..length {
                    let mut occurs = false;
                    for t in tasks.iter() {
                        if task_on_day(&t.start, &t.repetition, NaiveDate::from_yo(year, i)) {
                            occurs = true;
                        }
                    }
                    let day_justified = format!("{:>5} ", i);
                    let day_justified = if occurs { Yellow.bold().paint(day_justified).to_string() } else { String::from(day_justified) };
                    days[(i as usize - 1) % width].push_str(&day_justified);
                };
                for col in days.iter() {
                    println!("{}", col);
                }
            }
        }

    }

    pub fn get_start(values: Vec<u32>) -> Option<NaiveDate> {
        let current = Local::now().date();
        if values.len() == 1 {
            NaiveDate::from_ymd_opt(current.year(), values[0], current.day())
        } else if values.len() == 2 {
            NaiveDate::from_ymd_opt(current.year(), values[0], values[1])
        } else if values.len() >= 2 {
            NaiveDate::from_ymd_opt(values[2] as i32, values[0], values[1])
        } else {
            None
        }
    }

    pub fn date_or_today(values: Option<Vec<u32>>) -> NaiveDate {
        match values {
            Some(raw) => match get_start(raw) {
                Some(date) => date,
                None => Local::now().date().naive_local()
            },
            None => Local::now().date().naive_local()
        }
    }
}
