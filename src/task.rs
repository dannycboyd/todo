
    extern crate chrono;
    use chrono::prelude::*; // Utc, Local
    use chrono::Date;

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

    #[derive(Debug)]
    pub enum CustomRep {
        Weekly(MultiDays),
        EveryXDays(u32),
    }

    #[derive(Debug)]
    pub enum Repetition {
        Daily,
        Weekly,
        Monthly,
        Custom(CustomRep),
    }

    #[derive(Debug)]
    pub struct TaskItem {
        pub start: Date<Local>,
        pub repetition: Repetition,
        pub title: String,
        pub note: String,
        // completed: [DateTime<Utc>], // Should use a vector here, maybe. Other solution?
        pub finished: bool,
    }

    impl TaskItem {
        pub fn new(start: Date<Local>) -> TaskItem {
            TaskItem {
                start,
                repetition: Repetition::Daily,
                title: String::from("Title"),
                note: String::from(""),
                finished: false,
            }
        }
    }

//    impl TaskItem {
//        fn daysThisWeek(sunday: DateTime<Utc>)
//    }
