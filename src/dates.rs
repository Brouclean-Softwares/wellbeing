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
}
