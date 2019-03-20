
pub mod Calendar {
    use chrono::NaiveDate;
    use chrono::Local; // Utc, Local
    use chrono::Datelike;

    pub fn print_month(month: u32) {
        let year = Local::now().date().year();
        let current_month = NaiveDate::from_ymd(year, month, 1);
        let next_month = if (month == 12) {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        };
        let duration = next_month.signed_duration_since(current_month).num_days();
        let date = current_month.format("%B %Y").to_string();
        let len = date.len();

        println!("\n{:^width$}", date, width=21);
        let mut first = current_month.weekday().num_days_from_sunday(); // number of days from sunday
        println!("Su Mo Tu We Th Fr Sa");
        let mut line = String::from("");
        if first < 7 {
            for i in (0..first) {
                line.push_str("   ")
            }
        }
        for i in (0..duration) {
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

    pub fn print_week() {}

    pub fn get_monthday(day: u32) -> Option<NaiveDate> {
        let current = Local::now().date();
        NaiveDate::from_ymd_opt(current.year(), current.month(), day)
    }
}
