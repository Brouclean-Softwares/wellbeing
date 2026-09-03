use crate::AppState;
use crate::data::satisfying_moments::SatisfyingMoment;
use crate::data::users::ConnectedProfile;
use crate::dates::WithTimeZone;
use crate::locales::Localized;
use chrono::{Datelike, Months, NaiveDate};

pub struct JournalMonth {
    pub journal_days: Vec<JournalDay>,
}

impl JournalMonth {
    pub async fn from(state: &AppState, user_id: &i64, date_in_month: NaiveDate) -> Self {
        let month_first_day = date_in_month.with_day(1).unwrap();
        let next_month_first_day = month_first_day.checked_add_months(Months::new(1)).unwrap();

        let mut journal_days: Vec<JournalDay> = Vec::new();
        let mut day = month_first_day;

        while day < next_month_first_day {
            journal_days.push(JournalDay::from(state, user_id, day).await);

            day = day.succ_opt().unwrap();
        }

        Self { journal_days }
    }

    pub fn calendar(&self, localized: &impl Localized) -> Vec<Vec<Option<JournalDay>>> {
        let mut journal_iter = self.journal_days.iter();
        let mut journal_day = journal_iter.next();

        let first_day_offset = localized.week_day_number(&journal_day.unwrap().date) as usize - 1;

        let week_number =
            f64::ceil((self.journal_days.len() as f64 + first_day_offset as f64) / 7.0) as usize;

        let mut calendar: Vec<Vec<Option<JournalDay>>> = Vec::with_capacity(week_number);
        let mut week = Vec::with_capacity(7);

        for w in 0..week_number {
            for d in 0..7 {
                if w == 0 && d < first_day_offset {
                    week.push(None);
                } else {
                    week.push(journal_day.copied());
                    journal_day = journal_iter.next();
                }
            }

            calendar.push(week.clone());
            week = Vec::with_capacity(7);
        }

        calendar
    }
}

#[derive(Clone, Copy)]
pub struct JournalDay {
    pub date: NaiveDate,
    pub maximum_satisfaction_level: Option<i16>,
}

impl JournalDay {
    pub fn is_today(&self, connected_profile: &ConnectedProfile) -> bool {
        self.date.eq(&connected_profile.today())
    }

    pub async fn from(state: &AppState, user_id: &i64, date: NaiveDate) -> Self {
        let maximum_satisfaction_level =
            SatisfyingMoment::select_maximum_satisfaction_level_lived_at_for_user(
                state, user_id, &date,
            )
            .await
            .unwrap_or(None);

        Self {
            date,
            maximum_satisfaction_level,
        }
    }
}
