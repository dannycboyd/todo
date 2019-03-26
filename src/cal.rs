// repetition is defined here
// add functions for displaying repetition inside range
pub mod calendar {
    use chrono::NaiveDate;
    use chrono::Local; // Utc, Local
    use chrono::Datelike;
    use chrono::Duration;
    use crate::task::TaskItem;

    #[derive(Debug)]
    pub enum Repetition {
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

    // pub fn show_days_in_week(task: &TaskItem, start_date: NaiveDate) -> [u32; 7]) { // returns
        // given a vector of task refs and a start date,
        // for each task,
        // if occurs_on_day(task, day: naivedate)
        //      insert day into vec
        // print_week(start, days)
    // }

    // pub fn show_days_in_month(tasks: Vec<&Taskitem>, start_date: NaiveDate) {
        // using ansii terminal codes to highlight days
        // do print_month but with highlighted days
        // Given a vector of task refs and a month start day
        // for each task:
        //   if occurs_on_day(task, day: naivedate), insert day into vec
        // print_month(start: NaiveDate, days: Vec<u32>)
    // }

    // pub fn make_date(values: vec<u32>) {
    //  match values.len
    // 1: it's just a day in this month
    // 2: month/day
    // 3: month/day/year
    //}

    pub fn show_days_in_dur(tasks: Vec<&TaskItem>, start: NaiveDate, dur: u32) -> Vec<u32> {
        let mut days = vec![];
        for task in tasks.iter() {
            println!("{:?}", task);
            for i in 0..dur {
                match start.checked_add_signed(Duration::days(i as i64)) {
                    Some(check) => {
                        if task.occurs_on_day(check) { // this call zigzags, but i think it makes sense?
                            days.push(i);
                        }
                    },
                    None => println!("Something went wrong trying to add {} days to {:?}", i, &start),
                };
            };
        };
        println!("{:?}", days);
        days
    }

    pub fn task_on_day(start: &NaiveDate, rep: &Repetition, check: NaiveDate) -> bool {
        match rep {
            Repetition::Daily => true,
            Repetition::Weekly => start.weekday() == check.weekday(),
            Repetition::Monthly => start.day() == check.day(),
            _ => false
        }
    }

    pub fn print_month(date: NaiveDate) {
        // modify this to check all tasks and show days with tasks
        let year = date.year();
        let month = date.month();
        let current_month = NaiveDate::from_ymd(year, month, 1);
        let next_month = if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        };
        let duration = next_month.signed_duration_since(current_month).num_days();
        let date = current_month.format("%B %m|%Y").to_string();
        // let len = date.len();

        println!("\n{:^width$}", date, width=21);
        let mut first = current_month.weekday().num_days_from_sunday(); // number of days from sunday
        println!("Su Mo Tu We Th Fr Sa");
        let mut line = String::from("");
        if first < 7 {
            for _i in 0..first {
                line.push_str("   ")
            }
        }
        for i in 0..duration {
            first = (first + 1) % 7;
            // println!("day {}, first {}", i+1, first);
            line.push_str(&format!("{day:>2}", day=i+1));
            if first == 0 {
                println!("{}", line);
                line.clear();
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);

    }

    // pub fn print_week() {}

    pub fn get_monthday(day: u32) -> Option<NaiveDate> {
        let current = Local::now().date();
        NaiveDate::from_ymd_opt(current.year(), current.month(), day)
    }

    pub fn get_start(values: Vec<u32>) -> Option<NaiveDate> {
        let current = Local::now().date();
        if values.len() == 1 {
            NaiveDate::from_ymd_opt(current.year(), current.month(), values[0])
        } else if values.len() == 2 {
            NaiveDate::from_ymd_opt(current.year(), values[0], values[1])
        } else if values.len() >= 2 {
            NaiveDate::from_ymd_opt(values[2] as i32, values[0], values[1])
        } else {
            None
        }
    }
}
