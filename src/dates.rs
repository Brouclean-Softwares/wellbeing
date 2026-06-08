use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};

pub trait WithTimeZone {
    fn timezone(&self) -> impl TimeZone;

    fn now(&self) -> NaiveDateTime {
        Utc::now().with_timezone(&self.timezone()).naive_local()
    }

    fn today(&self) -> NaiveDate {
        Utc::now()
            .with_timezone(&self.timezone())
            .naive_local()
            .date()
    }

    fn previous_day(&self, date: &NaiveDate) -> NaiveDate {
        date.pred_opt()
            .expect("Should be able to process previous day")
    }

    fn next_day(&self, date: &NaiveDate) -> NaiveDate {
        date.succ_opt().expect("Should be able to process next day")
    }
}
