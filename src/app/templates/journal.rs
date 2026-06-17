use crate::app::templates::NavigationBar;
use crate::data::journal::{JournalDay, JournalMonth};
use crate::data::users::ConnectedProfile;
use crate::dates::{DateLevel, WithTimeZone};
use crate::locales::Translator;
use askama::Template;
use askama_web::WebTemplate;
use chrono::{Datelike, Months, NaiveDate};

fn url_param_with_day(date_level: DateLevel, day: Option<NaiveDate>) -> String {
    let date_level_string = serde_json::to_string(&date_level).unwrap();
    let date_level_string = date_level_string.trim_matches('"').to_string();

    match day {
        Some(day) => format!("?date_level={}&day={}", date_level_string, day.to_string()),
        None => format!("?date_level={}", date_level_string),
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "journal/journal_of_day_page.html")]
pub struct JournalOfDayPage {
    navigation_bar: NavigationBar,
    url_param_with_day: String,
}

impl JournalOfDayPage {
    pub fn from(connected_profile: ConnectedProfile, day: Option<NaiveDate>) -> JournalOfDayPage {
        let url_param_with_day = url_param_with_day(DateLevel::Day, day);

        Self {
            navigation_bar: NavigationBar {
                profile: connected_profile.clone().into(),
            },
            url_param_with_day,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "journal/journal_of_month_page.html")]
pub struct JournalOfMonthPage {
    navigation_bar: NavigationBar,
    url_param_with_day: String,
}

impl JournalOfMonthPage {
    pub fn from(connected_profile: ConnectedProfile, day: Option<NaiveDate>) -> JournalOfMonthPage {
        let url_param_with_day = url_param_with_day(DateLevel::Month, day);

        Self {
            navigation_bar: NavigationBar {
                profile: connected_profile.into(),
            },
            url_param_with_day,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "journal/day_navigation.html")]
pub struct DayNavigation {
    connected_profile: ConnectedProfile,
    previous_day: Option<NaiveDate>,
    day: NaiveDate,
    next_day: Option<NaiveDate>,
}

impl DayNavigation {
    pub fn from(connected_profile: ConnectedProfile, day: Option<NaiveDate>) -> Self {
        let today = connected_profile.today();

        let day = day.unwrap_or(today);

        let previous_day = if day > today {
            Some(today)
        } else {
            day.pred_opt()
        };

        let next_day = if day < today { day.succ_opt() } else { None };

        Self {
            connected_profile,
            previous_day,
            day,
            next_day,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "journal/month_navigation.html")]
pub struct MonthNavigation {
    connected_profile: ConnectedProfile,
    previous_month_date: Option<NaiveDate>,
    month_start_date: NaiveDate,
    next_month_date: Option<NaiveDate>,
    calendar: Vec<Vec<Option<JournalDay>>>,
}

impl MonthNavigation {
    pub fn from(connected_profile: ConnectedProfile, journal_month: JournalMonth) -> Self {
        let today = connected_profile.today();

        let month_start_date = journal_month.journal_days.first().unwrap().date;

        let month_end_date = month_start_date
            .checked_add_months(Months::new(1))
            .unwrap()
            .pred_opt()
            .unwrap();

        let previous_month_date = if month_start_date > today {
            Some(today)
        } else {
            month_start_date.pred_opt()
        };

        let next_month_date = if month_end_date < today {
            month_end_date.succ_opt()
        } else {
            None
        };

        let calendar = journal_month.calendar(&connected_profile);

        Self {
            connected_profile,
            previous_month_date,
            month_start_date,
            next_month_date,
            calendar,
        }
    }
}
