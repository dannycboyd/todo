// repetition is defined here
// add functions for displaying repetition inside range
pub mod calendar {
    use chrono::NaiveDate;
    use chrono::Local; // Utc, Local
    use chrono::Datelike;
    use chrono::Duration;

    use ansi_term::Style;
    use ansi_term::Color::{Yellow};

    use crate::task::TaskItem;

    #[derive(Debug)]
    pub enum Repetition {
        Never,
        Daily,
        Weekly,
        Monthly,
        Custom(MultiDays)
    }

    #[derive(Debug)]
    pub struct MultiDays {
        sun: bool,
        mon: bool,
        tue: bool,
        wed: bool,
        thur: bool,
        fri: bool,
        sat: bool,
    }

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
        let mut num_days = date.weekday().num_days_from_sunday();
        if num_days == 0 {
            Some(date)
        } else {
            let days: i32 = date.num_days_from_ce() - num_days as i32;
            NaiveDate::from_num_days_from_ce_opt(days)
        }
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
                let year = start.year();
                let mut month = start.month();
                let mut day: i64 = start.day() as i64;

                let diff = start.signed_duration_since(date).num_days();

                println!("Week of {}", start.format("%A, %B %d, %Y").to_string());
                for i in 0..7 {
                    if diff < 0 && date.month() != month && i + diff == 0 {
                        month = month + 1;
                        day = diff + 1;
                    }
                    let mut occurs = false;
                    for t in tasks.iter() {
                        if task_on_day(&t.start, &t.repetition, NaiveDate::from_ymd(year, month, (day + i) as u32)) {
                            happening.push(((day + i) as u32, t));
                            occurs = true;
                        }
                    }
                    let day_justified = format!("{:>} ", day+i);
                    let day_justified = if occurs { Yellow.bold().paint(day_justified).to_string() } else { String::from(day_justified) };
                    print!("{}", day_justified);
                }
                println!("");
                let mut day: u32 = 0;
                for (i, t) in happening.iter() {
                    if *i == day {
                        println!("    {:?}", t);
                    } else {
                        println!("{:>2}: {:?}", i, t);
                        day = *i;
                    }
                }
            }
        }
    }

    // pub fn print_week() {}
    //
    // pub fn get_monthday(day: u32) -> Option<NaiveDate> {
    //     let current = Local::now().date();
    //     NaiveDate::from_ymd_opt(current.year(), current.month(), day)
    // }

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
}
